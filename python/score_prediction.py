import glob
import os

import numpy as np
import pandas as pd
import torch
import torch.nn as nn

DATA_GLOB = "data/gen_*.csv"
WEIGHTS_PATH = "src/evaluators/weights.bin"   # Rust reads this
CKPT_PATH = "python/score_model.pt"           # resume point between runs
SCALE = 10.0      # target is (future points / SCALE); Rust must use the same number
EPOCHS = 3
BATCH_SIZE = 1024
LR = 3e-4
N_FEATURES = 194

device = "cuda" if torch.cuda.is_available() else "mps" if torch.backends.mps.is_available() else "cpu"

# ---------- Load data: column 0 = future points, rest = features ----------
files = sorted(glob.glob(DATA_GLOB))
data = np.concatenate(
    [pd.read_csv(f, header=None, dtype=np.float32).to_numpy() for f in files]
)
X = torch.from_numpy(data[:, 1:])
y = torch.from_numpy(data[:, 0]).unsqueeze(1) / SCALE
filled = (X[:, 19:].reshape(-1, 25, 7)[:, :, :6].sum(dim=2) > 0).sum(dim=1)
mean_by_filled = torch.zeros(26)
for k in range(26):
    m = filled == k
    if m.any():
        mean_by_filled[k] = y[m].mean()
y = y - mean_by_filled[filled].unsqueeze(1)
del data
assert X.shape[1] == N_FEATURES, f"expected {N_FEATURES} features, got {X.shape[1]}"
print(f"Training on {len(X)} rows from {len(files)} files "
      f"(future-points std: {y.std().item() * SCALE:.2f})")

# ---------- Model (warm start if a checkpoint exists) ----------
model = nn.Sequential(
    nn.Linear(N_FEATURES, 256), nn.ReLU(),
    nn.Linear(256, 128), nn.ReLU(),
    nn.Linear(128, 64), nn.ReLU(),
    nn.Linear(64, 1),
)
if os.path.exists(CKPT_PATH):
    model.load_state_dict(torch.load(CKPT_PATH, map_location="cpu"))
model.to(device)

opt = torch.optim.Adam(model.parameters(), lr=LR)
loss_fn = nn.MSELoss()

# ---------- Train ----------
model.train()
for epoch in range(EPOCHS):
    order = torch.randperm(len(X))
    total = 0.0
    for i in range(0, len(order), BATCH_SIZE):
        idx = order[i:i + BATCH_SIZE]
        xb, yb = X[idx].to(device), y[idx].to(device)
        opt.zero_grad()
        loss = loss_fn(model(xb), yb)
        loss.backward()
        opt.step()
        total += loss.item() * len(idx)
    rmse = (total / len(order)) ** 0.5 * SCALE
    print(f"  epoch {epoch + 1}/{EPOCHS} | RMSE {rmse:.2f} points")

# ---------- Save checkpoint + weights for Rust ----------
model.cpu()
os.makedirs(os.path.dirname(CKPT_PATH), exist_ok=True)
torch.save(model.state_dict(), CKPT_PATH)

sd = model.state_dict()
tmp = WEIGHTS_PATH + ".tmp"
with open(tmp, "wb") as f:
    for i in (0, 2, 4, 6):  # Linear layers inside the Sequential
        f.write(sd[f"{i}.weight"].numpy().astype("<f4").tobytes())
        f.write(sd[f"{i}.bias"].numpy().astype("<f4").tobytes())
os.replace(tmp, WEIGHTS_PATH)  # swap in one step so Rust never sees a half-written file
print("Saved", WEIGHTS_PATH)
