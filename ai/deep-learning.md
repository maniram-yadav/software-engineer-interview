# Deep Learning — Complete Theory & TensorFlow Implementation Guide

A ground-up reference: the math, the intuition, why each technique exists, when to reach for it, and how to implement it in TensorFlow/Keras. Read top to bottom to build a full mental model, or jump to a section before an interview or before writing code.

---

## Table of Contents

1. Neural Network Fundamentals
2. Activation Functions
3. Loss Functions
4. Backpropagation & Gradient Descent
5. Optimizers
6. Weight Initialization
7. Regularization
8. Normalization Layers
9. Learning Rate Scheduling
10. Convolutional Neural Networks (CNNs)
11. Classic & Modern CNN Architectures
12. Recurrent Networks — RNN, LSTM, GRU
13. Transformers & Attention (deep learning lens)
14. Autoencoders & Representation Learning
15. Generative Models — VAEs, GANs, Diffusion
16. Transfer Learning & Fine-Tuning
17. Data Pipelines & Augmentation
18. Hyperparameter Tuning
19. Evaluation Metrics
20. Bias-Variance, Overfitting & Debugging Training
21. Distributed & Mixed-Precision Training
22. TensorFlow/Keras Practical Patterns
23. "Which Algorithm/Architecture Do I Use?" Cheat Sheet

---

## 1. Neural Network Fundamentals

A neural network is a composition of parameterized linear transformations and nonlinear activations that approximates a function `f: X → Y`.

**Perceptron (single unit):**

```
z = w·x + b
a = g(z)          # g = activation function
```

**Multi-Layer Perceptron (MLP):** stack layers so each layer's output feeds the next:

```
h1 = g1(W1·x  + b1)
h2 = g2(W2·h1 + b2)
y  = g3(W3·h2 + b3)
```

**Universal Approximation Theorem:** a feedforward network with a single hidden layer of finite width and a nonlinear activation can approximate any continuous function on a compact domain to arbitrary precision — *in theory*. In practice, depth (many narrow layers) is far more parameter-efficient than width (one huge layer) because deep networks build hierarchical, reusable feature representations (edges → shapes → parts → objects, in vision; characters → words → phrases → meaning, in text).

**Why nonlinearity matters:** without an activation function, stacking linear layers collapses to a single linear layer (`W2·(W1·x) = (W2·W1)·x`), so the network could only ever learn linear decision boundaries.

**Minimal TensorFlow MLP:**

```python
import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers

model = keras.Sequential([
    layers.Input(shape=(20,)),
    layers.Dense(64, activation="relu"),
    layers.Dense(32, activation="relu"),
    layers.Dense(1, activation="sigmoid"),   # binary classification head
])

model.compile(optimizer="adam", loss="binary_crossentropy", metrics=["accuracy"])
model.summary()
```

**Interview angle:** be able to explain *why* depth > width empirically, and derive the parameter count of a Dense layer (`in_dim * out_dim + out_dim` biases).

---

## 2. Activation Functions

The activation function introduces nonlinearity and shapes gradient flow. Choice affects convergence speed, vanishing/exploding gradients, and output range.

| Function | Formula | Range | Notes |
|---|---|---|---|
| **Sigmoid** | `1/(1+e^-z)` | (0,1) | Saturates for \|z\|>4 → vanishing gradients. Use only on output layer for binary probability. |
| **Tanh** | `(e^z-e^-z)/(e^z+e^-z)` | (-1,1) | Zero-centered (better than sigmoid for hidden layers) but still saturates. Common in RNN gates. |
| **ReLU** | `max(0, z)` | [0,∞) | Default for hidden layers. Cheap, no saturation for z>0. Risk: "dying ReLU" (neuron stuck outputting 0 forever if it enters the negative regime with large negative bias). |
| **Leaky ReLU** | `z if z>0 else αz` (α≈0.01) | (-∞,∞) | Fixes dying ReLU by allowing a small negative gradient. |
| **ELU** | `z if z>0 else α(e^z-1)` | (-α,∞) | Smooth, negative saturation helps push mean activations toward 0 (faster convergence), costlier (exp). |
| **GELU** | `z·Φ(z)` (Gaussian CDF) | (-∞,∞) | Smooth, probabilistic gating. Default in Transformers (BERT, GPT, ViT). |
| **Swish/SiLU** | `z·sigmoid(z)` | (-∞,∞) | Found via NAS (Google). Used in EfficientNet. Smooth, non-monotonic. |
| **Softmax** | `e^zi / Σe^zj` | (0,1), sums to 1 | Output layer for multi-class classification — converts logits to a probability distribution. |

**Rules of thumb:**
- Hidden layers in CNNs/MLPs → **ReLU** (or a variant if you observe dead neurons).
- Transformers → **GELU** or **SiLU/Swish**.
- Output layer: **sigmoid** for binary/multi-label, **softmax** for multi-class single-label, **linear (none)** for regression.
- RNN gates → **sigmoid** (for gates, need 0-1 range) and **tanh** (for candidate state, need -1 to 1).

```python
# Comparing activations in Keras
layers.Dense(64, activation="relu")
layers.Dense(64, activation="gelu")
layers.Dense(64, activation=tf.nn.silu)          # Swish
layers.LeakyReLU(alpha=0.1)                       # as a standalone layer, after a linear Dense(activation=None)
```

**Vanishing gradient connection:** `sigmoid'(z) = sigmoid(z)(1-sigmoid(z))`, max value 0.25 — chained across many layers via the chain rule, gradients shrink exponentially. This is the core reason ReLU-family activations replaced sigmoid/tanh in deep hidden stacks.

---

## 3. Loss Functions

The loss function defines what "good" means numerically and is what gradients are computed against.

**Regression:**
- **MSE** `(1/n)Σ(y-ŷ)²` — penalizes large errors heavily (quadratic), sensitive to outliers, smooth gradients everywhere (good for gradient descent).
- **MAE** `(1/n)Σ|y-ŷ|` — robust to outliers, but gradient is constant (±1) so it doesn't slow near the optimum — can cause oscillation near convergence.
- **Huber loss** — quadratic for small errors, linear for large errors (delta threshold). Combines MSE's smooth convergence with MAE's outlier robustness. Preferred in RL and noisy-label regression.

**Classification:**
- **Binary Cross-Entropy** `-(y·log(ŷ) + (1-y)·log(1-ŷ))` — pair with sigmoid output.
- **Categorical Cross-Entropy** `-Σ yi·log(ŷi)` — pair with softmax output, one-hot labels.
- **Sparse Categorical Cross-Entropy** — same as above but labels are integers, not one-hot (saves memory, avoids explicit one-hot encoding).
- **Focal Loss** `-α(1-ŷ)^γ·log(ŷ)` — down-weights easy, well-classified examples so the model focuses on hard/rare examples. Standard for **class-imbalanced** detection tasks (e.g., RetinaNet).
- **KL Divergence** `Σ p(x)·log(p(x)/q(x))` — measures how one probability distribution diverges from another. Used in VAEs (latent regularization), knowledge distillation, RLHF (policy vs. reference model).
- **Contrastive / Triplet Loss** — used in metric learning / embeddings (face recognition, similarity search): pulls same-class embeddings together, pushes different-class embeddings apart by a margin.

```python
# Regression
model.compile(optimizer="adam", loss="mse", metrics=["mae"])
model.compile(optimizer="adam", loss=keras.losses.Huber(delta=1.0))

# Multi-class, one-hot labels
model.compile(optimizer="adam", loss="categorical_crossentropy")

# Multi-class, integer labels (much more common in practice)
model.compile(optimizer="adam", loss="sparse_categorical_crossentropy")

# Imbalanced binary classification
def focal_loss(gamma=2.0, alpha=0.25):
    def loss_fn(y_true, y_pred):
        y_pred = tf.clip_by_value(y_pred, 1e-7, 1 - 1e-7)
        ce = -(y_true * tf.math.log(y_pred) + (1 - y_true) * tf.math.log(1 - y_pred))
        p_t = y_true * y_pred + (1 - y_true) * (1 - y_pred)
        return tf.reduce_mean(alpha * tf.pow(1 - p_t, gamma) * ce)
    return loss_fn

model.compile(optimizer="adam", loss=focal_loss())
```

**Interview angle:** know *why* cross-entropy (not MSE) is standard for classification — MSE + sigmoid produces a non-convex loss surface with vanishing gradients at saturation; cross-entropy's gradient w.r.t. logits simplifies to `(ŷ - y)`, giving strong, well-behaved gradients even when the prediction is confidently wrong.

---

## 4. Backpropagation & Gradient Descent

**Forward pass:** compute predictions and loss by pushing input through the network.

**Backward pass (backpropagation):** apply the **chain rule** to compute `∂L/∂w` for every weight, by propagating gradients from the loss backward through each layer's local Jacobian.

```
∂L/∂W1 = ∂L/∂y · ∂y/∂h2 · ∂h2/∂h1 · ∂h1/∂W1
```

Each layer only needs to know its local derivative and the gradient flowing in from the layer above — this is what makes backprop `O(n)` in the number of layers instead of exponential.

**Gradient Descent update rule:**

```
w := w - η · ∂L/∂w        # η = learning rate
```

**Variants by batch size:**
- **Batch GD** — full dataset per step. Stable gradient, but slow and memory-heavy; impractical beyond small datasets.
- **Stochastic GD (SGD)** — one example per step. Fast, noisy updates (noise can help escape shallow local minima) but unstable.
- **Mini-batch GD** — the practical default (e.g., 32–512 examples/step). Balances gradient stability and compute efficiency, and maps well onto GPU/TPU parallelism.

**Vanishing/Exploding gradients:** in deep networks, gradients are products of many layer-wise Jacobians. If those terms are consistently <1, gradients vanish (early layers barely update); if consistently >1, gradients explode (loss becomes NaN). Mitigations: ReLU-family activations, proper weight initialization (§6), normalization layers (§8), residual/skip connections, gradient clipping.

```python
# Manual gradient computation with tf.GradientTape (how Keras `fit` works under the hood)
optimizer = keras.optimizers.Adam(learning_rate=1e-3)
loss_fn = keras.losses.SparseCategoricalCrossentropy()

@tf.function
def train_step(x_batch, y_batch):
    with tf.GradientTape() as tape:
        preds = model(x_batch, training=True)
        loss = loss_fn(y_batch, preds)
    grads = tape.gradient(loss, model.trainable_variables)
    grads, _ = tf.clip_by_global_norm(grads, clip_norm=1.0)   # exploding-gradient guard
    optimizer.apply_gradients(zip(grads, model.trainable_variables))
    return loss
```

---

## 5. Optimizers

All optimizers solve the same problem — using the gradient to update weights — but differ in how they use *gradient history* to adapt step size and direction.

**SGD (vanilla):** `w := w - η·g`. Simple, needs careful LR tuning, tends to generalize well but converges slowly and can zig-zag across narrow ravines in the loss surface.

**SGD + Momentum:** accumulates a velocity vector so updates keep moving in a consistent direction, damping oscillation:
```
v := βv + g          (β ≈ 0.9)
w := w - ηv
```

**Nesterov Momentum:** looks ahead — computes the gradient at the "lookahead" position `w - βv` rather than at `w`, giving a slight correction that improves convergence.

**Adagrad:** scales the LR per-parameter, inversely proportional to the sum of squared past gradients — great for sparse features (NLP with sparse embeddings), but the accumulated sum only grows, so the effective LR eventually shrinks to near zero and training stalls.

**RMSprop:** fixes Adagrad's decay problem by using an **exponential moving average** of squared gradients instead of a running sum:
```
s := βs + (1-β)g²
w := w - η·g/√(s+ε)
```
Good default for RNNs.

**Adam (Adaptive Moment Estimation):** combines momentum (1st moment) + RMSprop-style per-parameter scaling (2nd moment), with bias correction for the early steps:
```
m := β1·m + (1-β1)·g                 (β1 ≈ 0.9)
s := β2·s + (1-β2)·g²                (β2 ≈ 0.999)
m̂ := m / (1-β1^t) ;  ŝ := s / (1-β2^t)     # bias correction
w := w - η · m̂/(√ŝ + ε)
```
**Default choice for most deep learning** — fast convergence, robust to LR choice, works well out of the box.

**AdamW:** Adam with **decoupled weight decay** (L2 regularization applied directly to the weight update, not folded into the gradient like vanilla Adam does). Fixes a subtle bug where Adam's adaptive scaling interacts badly with L2-as-gradient-penalty. **Standard optimizer for Transformers/LLMs** today.

**LAMB / LARS:** layer-wise adaptive LR scaling designed for very large batch sizes (thousands+) used in large-scale distributed pretraining, where standard Adam becomes unstable.

| Optimizer | Best for | TF class |
|---|---|---|
| SGD + momentum | CNNs where max generalization matters (ResNet on ImageNet, with LR schedule) | `keras.optimizers.SGD(momentum=0.9, nesterov=True)` |
| RMSprop | RNNs, older architectures | `keras.optimizers.RMSprop()` |
| Adam | Default / prototyping / most tasks | `keras.optimizers.Adam()` |
| AdamW | Transformers, LLM fine-tuning | `keras.optimizers.AdamW(weight_decay=0.01)` |

```python
optimizer = keras.optimizers.AdamW(learning_rate=3e-4, weight_decay=0.01)
model.compile(optimizer=optimizer, loss="sparse_categorical_crossentropy", metrics=["accuracy"])
```

**Interview angle:** know why AdamW ≠ Adam + L2 regularization added to the loss — the difference (decoupling) is a common gotcha question.

---

## 6. Weight Initialization

Bad initialization causes vanishing/exploding activations and gradients before training even starts, because it changes the variance of each layer's output.

- **Zero init** — never do this for weights; every neuron computes the same gradient (symmetry never breaks). Biases can be zero-initialized safely.
- **Xavier/Glorot init** — variance `= 2/(fan_in + fan_out)`. Designed for **sigmoid/tanh** activations to keep activation variance roughly constant across layers.
- **He init** — variance `= 2/fan_in`. Designed for **ReLU family** (accounts for the fact ReLU zeroes out ~half the activations, so needs more initial variance to compensate).
- **LeCun init** — variance `= 1/fan_in`. Used with **SELU** activation for self-normalizing networks.

```python
layers.Dense(64, activation="relu", kernel_initializer="he_normal")
layers.Dense(64, activation="tanh", kernel_initializer="glorot_uniform")   # Keras default
```

In practice, Keras defaults (Glorot) are fine for most cases; explicitly set He initialization when using ReLU/Leaky ReLU in very deep networks.

---

## 7. Regularization

Techniques that reduce overfitting by constraining the model's effective capacity or injecting noise.

**L1 regularization** — adds `λΣ|w|` to the loss. Drives many weights to exactly zero → sparse models, implicit feature selection.

**L2 regularization (weight decay)** — adds `λΣw²` to the loss. Shrinks weights smoothly toward zero, discourages any single weight from becoming too large. Most common default.

**Dropout** — during training, randomly zero out a fraction `p` of activations each forward pass, then scale remaining activations by `1/(1-p)` (inverted dropout) so expected output magnitude is preserved. Forces the network to not rely on any single neuron (an implicit ensemble of sub-networks). Disabled at inference.

```python
layers.Dense(128, activation="relu"),
layers.Dropout(0.3),
```

**Early Stopping** — stop training when validation loss stops improving, preventing the model from memorizing training noise in later epochs.

```python
callback = keras.callbacks.EarlyStopping(monitor="val_loss", patience=5, restore_best_weights=True)
model.fit(X_train, y_train, validation_split=0.2, epochs=100, callbacks=[callback])
```

**Data Augmentation** — regularizes by expanding the effective training distribution (see §17).

**Label Smoothing** — instead of one-hot targets `[0,1,0]`, use `[0.033, 0.933, 0.033]`. Prevents the model from becoming overconfident, improves calibration and generalization. Standard in modern image classifiers and some Transformer training recipes.

```python
loss = keras.losses.CategoricalCrossentropy(label_smoothing=0.1)
```

**Weight decay via optimizer:**
```python
keras.optimizers.AdamW(learning_rate=1e-3, weight_decay=1e-4)
```
Or via layer-level regularizer:
```python
layers.Dense(64, kernel_regularizer=keras.regularizers.l2(1e-4))
```

---

## 8. Normalization Layers

Normalization stabilizes and accelerates training by controlling the distribution of activations flowing through the network.

**Batch Normalization** — normalizes activations across the **batch dimension** for each feature: `(x - batch_mean) / sqrt(batch_var + ε)`, then applies learnable scale `γ` and shift `β`. Reduces internal covariate shift, allows higher learning rates, acts as a mild regularizer. **Downside:** behavior depends on batch statistics, so it's unstable with very small batch sizes and behaves differently in train vs. inference (uses running averages at inference). Standard in CNNs.

```python
layers.Conv2D(64, 3, padding="same"),
layers.BatchNormalization(),
layers.ReLU(),
```

**Layer Normalization** — normalizes across the **feature dimension** for each individual sample, independent of batch size. No train/inference discrepancy. Standard in Transformers/RNNs, where batch statistics are less meaningful (variable sequence length, autoregressive generation with batch size 1).

```python
layers.LayerNormalization(epsilon=1e-6)
```

**Group Normalization** — divides channels into groups and normalizes within each group per sample. Useful when batch size must be small (e.g., high-res segmentation/detection) where BatchNorm becomes unstable.

**Rule of thumb:** CNNs → BatchNorm. Transformers/sequence models → LayerNorm. Small-batch vision tasks (detection/segmentation) → GroupNorm.

---

## 9. Learning Rate Scheduling

The single highest-leverage hyperparameter. Too high → divergence/instability. Too low → painfully slow convergence, gets stuck in sharp minima.

- **Step decay** — drop LR by a factor every N epochs (e.g., ×0.1 every 30 epochs). Simple, used in classic ResNet training.
- **Exponential decay** — `η_t = η0 · decay_rate^(t/decay_steps)`.
- **Cosine annealing** — smoothly decays LR following a cosine curve to near-zero; very popular for training from scratch — tends to find flatter, better-generalizing minima.
- **Warmup** — linearly ramp LR up from ~0 for the first N steps before applying the main schedule. Essential for Transformers — without warmup, Adam's early variance estimates are unreliable and large early updates destabilize training.
- **ReduceLROnPlateau** — reactively drop LR when validation loss stalls. Good default when you don't want to hand-tune a schedule.
- **Cyclical LR / One-Cycle Policy** (Leslie Smith) — oscillate LR between bounds, often converges faster and to better minima than monotonic decay; used heavily in fast.ai-style training recipes.

```python
# Cosine decay with warmup (typical Transformer recipe)
lr_schedule = keras.optimizers.schedules.CosineDecay(
    initial_learning_rate=1e-4, decay_steps=10_000, warmup_target=3e-4, warmup_steps=1_000
)
optimizer = keras.optimizers.AdamW(learning_rate=lr_schedule)

# Reactive plateau-based reduction
callback = keras.callbacks.ReduceLROnPlateau(monitor="val_loss", factor=0.5, patience=3, min_lr=1e-6)
```

---

## 10. Convolutional Neural Networks (CNNs)

CNNs exploit the spatial structure of images (and grid-like data generally) via three key ideas: **local receptive fields**, **parameter sharing**, and **translation equivariance**.

**Convolution operation:** slide a small learnable filter (kernel) over the input, computing a dot product at each position:
```
output[i,j] = Σ_m Σ_n  input[i+m, j+n] · kernel[m,n]
```
Each filter learns to detect one pattern (edge, texture, color blob) regardless of *where* it appears in the image — this parameter sharing is what makes CNNs vastly more sample-efficient than a Dense layer on raw pixels.

**Key hyperparameters:**
- **Kernel size** — typically 3×3 (stacking multiple 3×3 convs approximates a larger receptive field with fewer parameters than one big kernel — this insight drove VGG's design).
- **Stride** — step size; stride 2 halves spatial resolution (used instead of/alongside pooling for downsampling in modern nets).
- **Padding** — `"same"` preserves spatial size, `"valid"` shrinks it (no padding).
- **Channels/filters** — number of independent feature detectors per layer; typically increases with depth as spatial resolution shrinks.

**Pooling** — downsamples feature maps to reduce computation and add local translation invariance. **Max pooling** (keep strongest activation) is standard; **average pooling** is common at the final layer before classification (Global Average Pooling — replaces flattening + huge Dense layer, drastically cutting parameters and overfitting risk).

**Receptive field:** the region of the input that influences a given output activation. Grows with depth and stride; deep networks need large receptive fields to "see" whole objects.

```python
from tensorflow.keras import layers, models

model = models.Sequential([
    layers.Input(shape=(224, 224, 3)),
    layers.Conv2D(32, 3, padding="same", activation="relu"),
    layers.BatchNormalization(),
    layers.MaxPooling2D(2),

    layers.Conv2D(64, 3, padding="same", activation="relu"),
    layers.BatchNormalization(),
    layers.MaxPooling2D(2),

    layers.Conv2D(128, 3, padding="same", activation="relu"),
    layers.BatchNormalization(),
    layers.GlobalAveragePooling2D(),   # instead of Flatten + big Dense

    layers.Dense(10, activation="softmax"),
])
```

**When to use CNNs:** any grid-structured data with local correlation and translation invariance — images, spectrograms (audio), some time series (1D conv), even certain graph-like/structured tabular problems. Not the right tool for long-range sequential dependency modeling (that's RNN/Transformer territory) or unordered set data (that's GNN/DeepSets territory).

---

## 11. Classic & Modern CNN Architectures

Understanding *why* each architecture was introduced matters more than memorizing layer counts.

- **LeNet-5 (1998)** — the original CNN, digit recognition. Conv → pool → conv → pool → dense.
- **AlexNet (2012)** — proved deep CNNs + GPU training + ReLU + dropout could crush ImageNet. Kickstarted the deep learning era.
- **VGG (2014)** — showed depth matters; used only stacked 3×3 convs. Simple but parameter-heavy (~138M params for VGG16), mostly superseded but still used as a feature extractor / perceptual loss backbone.
- **ResNet (2015)** — introduced **residual/skip connections**: `output = F(x) + x`. This lets gradients flow directly through the identity path during backprop, solving the degradation problem where very deep plain networks got *worse* training accuracy than shallower ones (not due to overfitting — due to optimization difficulty). Enabled networks with 100+ layers. **This is the single most important architectural idea in modern deep learning** — it reappears in Transformers, U-Net, and nearly every deep architecture since.
- **Inception/GoogLeNet** — runs multiple kernel sizes (1×1, 3×3, 5×5) in parallel per block and concatenates results, capturing multi-scale features efficiently; uses 1×1 convs as a "bottleneck" to reduce channel dimensionality cheaply before expensive convolutions.
- **MobileNet** — **depthwise separable convolutions** (a spatial conv per-channel, followed by a 1×1 conv to mix channels) cut compute ~8-9x vs. standard convolution with minimal accuracy loss. Designed for mobile/edge inference.
- **EfficientNet** — systematically scales depth, width, *and* resolution together (compound scaling) according to a searched ratio, rather than arbitrarily scaling one dimension; found via neural architecture search (NAS).
- **U-Net** — encoder-decoder with skip connections between corresponding encoder/decoder resolutions, preserving fine spatial detail lost during downsampling. The standard architecture for **image segmentation** and the backbone of diffusion model denoisers (Stable Diffusion's UNet).
- **Vision Transformer (ViT)** — splits an image into patches, treats each patch as a "token," and applies a standard Transformer encoder. Outperforms CNNs at scale (large data + large compute) because it lacks CNNs' built-in translation-invariance bias, but that same lack of inductive bias makes it need more data to reach the same performance from scratch — hence ViTs are usually pretrained on huge datasets or use hybrid CNN-stem designs.

```python
# Residual block (the core ResNet idea) in Keras functional API
def residual_block(x, filters):
    shortcut = x
    x = layers.Conv2D(filters, 3, padding="same", activation="relu")(x)
    x = layers.BatchNormalization()(x)
    x = layers.Conv2D(filters, 3, padding="same")(x)
    x = layers.BatchNormalization()(x)
    if shortcut.shape[-1] != filters:
        shortcut = layers.Conv2D(filters, 1, padding="same")(shortcut)   # projection shortcut
    x = layers.Add()([x, shortcut])
    return layers.ReLU()(x)

inputs = keras.Input(shape=(32, 32, 3))
x = layers.Conv2D(64, 3, padding="same", activation="relu")(inputs)
x = residual_block(x, 64)
x = residual_block(x, 128)
outputs = layers.Dense(10, activation="softmax")(layers.GlobalAveragePooling2D()(x))
model = keras.Model(inputs, outputs)
```

**Using pretrained architectures directly (most real projects do this instead of training from scratch — see §16):**
```python
base = keras.applications.EfficientNetV2B0(include_top=False, weights="imagenet", input_shape=(224,224,3))
```

---

## 12. Recurrent Networks — RNN, LSTM, GRU

RNNs process sequences by maintaining a **hidden state** updated at each timestep, so the same weights are reused across time (parameter sharing across the sequence dimension, analogous to CNN's spatial parameter sharing).

**Vanilla RNN:**
```
h_t = tanh(Wx·x_t + Wh·h_{t-1} + b)
```
**Problem:** the chain rule through `T` timesteps multiplies `T` copies of `Wh` and the activation derivative — if the dominant eigenvalue of `Wh` is <1, gradients vanish exponentially over long sequences (the network "forgets" long-range dependencies); if >1, gradients explode.

**LSTM (Long Short-Term Memory)** — fixes vanishing gradients with a separate **cell state** `C_t` that flows across timesteps through mostly-linear operations (additive updates, not repeated multiplication), gated by three learned sigmoid gates:
- **Forget gate** `f_t` — how much of the old cell state to keep.
- **Input gate** `i_t` — how much of the new candidate value to write in.
- **Output gate** `o_t` — how much of the cell state to expose as the hidden state.

```
f_t = σ(Wf·[h_{t-1}, x_t] + bf)
i_t = σ(Wi·[h_{t-1}, x_t] + bi)
C̃_t = tanh(Wc·[h_{t-1}, x_t] + bc)
C_t = f_t * C_{t-1} + i_t * C̃_t          # additive update = gradient highway
o_t = σ(Wo·[h_{t-1}, x_t] + bo)
h_t = o_t * tanh(C_t)
```

**GRU (Gated Recurrent Unit)** — a simplified LSTM: merges cell and hidden state into one, uses two gates (reset, update) instead of three. Fewer parameters, often comparable performance, faster to train — a reasonable default when LSTM feels like overkill.

```python
model = keras.Sequential([
    layers.Input(shape=(100, 300)),                    # (timesteps, features)
    layers.Bidirectional(layers.LSTM(128, return_sequences=True)),
    layers.LSTM(64),
    layers.Dense(1, activation="sigmoid"),
])
```

**When to use RNN/LSTM/GRU vs. Transformers today:** Transformers have largely replaced RNNs for NLP and most sequence tasks because self-attention gives direct O(1)-hop access to any past token (vs. RNNs' O(n) sequential propagation) and trains fully in parallel. RNNs/LSTMs still make sense when: (a) sequences are very long and strictly streaming/online (constant memory per step, no need to store a growing KV cache), (b) the dataset is small (Transformers are data-hungry due to weaker inductive bias), (c) low-latency edge/embedded inference is required, or (d) the task is a classic time-series forecasting problem where a lighter model suffices.

---

## 13. Transformers & Attention (deep learning lens)

Full attention math and LLM-specific detail lives in `ai/genai_interview_guide.md` (topics 2-7) — here's the deep-learning-training angle specifically.

**Why Transformers train faster than RNNs in wall-clock time:** the entire sequence is processed in one matrix-multiply-heavy forward pass (no sequential dependency across timesteps within a layer), which parallelizes perfectly on GPU/TPU.

**Positional encoding is required** because self-attention itself is permutation-invariant (it has no notion of order) — sinusoidal encodings (original paper) or learned/rotary (RoPE, used in Llama/GPT-NeoX-style models) inject position information.

**Training-specific details:**
- **Pre-LN vs Post-LN:** modern Transformers apply LayerNorm *before* the attention/FFN sublayer (`x + Sublayer(LN(x))`) rather than after, which gives much more stable gradients at initialization and removes the need for careful warmup in some setups — this is why virtually every modern LLM uses pre-norm.
- **Label smoothing + AdamW + cosine/linear decay with warmup** is the standard training recipe.
- **Teacher forcing:** during training, the decoder is fed the ground-truth previous token (not its own prediction) so training remains parallelizable and doesn't compound early mistakes — creates a train/inference mismatch ("exposure bias") that techniques like scheduled sampling partially address.

```python
# Minimal Transformer encoder block in Keras
def transformer_block(x, num_heads, key_dim, ff_dim, dropout=0.1):
    attn_out = layers.MultiHeadAttention(num_heads=num_heads, key_dim=key_dim)(x, x)
    attn_out = layers.Dropout(dropout)(attn_out)
    x = layers.LayerNormalization(epsilon=1e-6)(x + attn_out)          # pre/post-norm residual

    ffn = keras.Sequential([
        layers.Dense(ff_dim, activation="gelu"),
        layers.Dense(x.shape[-1]),
    ])
    ffn_out = layers.Dropout(dropout)(ffn(x))
    return layers.LayerNormalization(epsilon=1e-6)(x + ffn_out)

inputs = keras.Input(shape=(128, 256))     # (seq_len, embed_dim)
x = transformer_block(inputs, num_heads=8, key_dim=32, ff_dim=1024)
```

---

## 14. Autoencoders & Representation Learning

**Autoencoder (AE):** an encoder compresses input `x` to a low-dimensional latent `z`, a decoder reconstructs `x̂` from `z`, trained to minimize reconstruction loss (`MSE(x, x̂)`). Forces the bottleneck to learn a compact, information-dense representation.

**Uses:** dimensionality reduction (nonlinear alternative to PCA), denoising (train to reconstruct clean input from corrupted input — "Denoising Autoencoder"), anomaly detection (anomalies reconstruct poorly since the model only learned the normal-data manifold), pretraining a feature extractor for downstream tasks.

```python
latent_dim = 32
encoder = keras.Sequential([
    layers.Input(shape=(784,)),
    layers.Dense(256, activation="relu"),
    layers.Dense(latent_dim, activation="relu"),
])
decoder = keras.Sequential([
    layers.Input(shape=(latent_dim,)),
    layers.Dense(256, activation="relu"),
    layers.Dense(784, activation="sigmoid"),
])
autoencoder = keras.Sequential([encoder, decoder])
autoencoder.compile(optimizer="adam", loss="mse")
```

**Interview angle:** distinguish a plain autoencoder (deterministic latent, no generative sampling guarantee — the latent space has "holes" you can't meaningfully sample from) from a VAE (§15), which is explicitly regularized to be a smooth, sample-able probability distribution.

---

## 15. Generative Models — VAEs, GANs, Diffusion

### Variational Autoencoders (VAE)
Instead of encoding `x` to a single point `z`, a VAE encodes to a **distribution** — mean `μ` and log-variance `logσ²` — and samples `z ~ N(μ, σ²)` via the **reparameterization trick** (`z = μ + σ·ε`, `ε ~ N(0,1)`) so gradients can still flow through the sampling step.

**Loss = Reconstruction loss + KL Divergence** (regularizes the latent distribution toward a standard normal `N(0,1)`, keeping the latent space smooth and continuous so any sampled `z` decodes to something plausible):
```
L = E[log p(x|z)]  -  β·KL(q(z|x) || N(0,1))
```
The `β` term (β-VAE) trades off reconstruction fidelity vs. latent disentanglement.

**Use when:** you need a smooth, interpretable, sample-able latent space (interpolation between data points, controllable generation) and training stability matters more than maximum sample sharpness.

### Generative Adversarial Networks (GAN)
Two networks compete: a **Generator** `G` maps noise `z` to fake samples `G(z)`; a **Discriminator** `D` tries to distinguish real samples from `G(z)`. Trained as a minimax game:
```
min_G max_D  E[log D(x)] + E[log(1 - D(G(z)))]
```
`D` pushes `G` to produce increasingly realistic samples; at the (theoretical) optimum, `D` outputs 0.5 everywhere (can't tell real from fake).

**Known training issues:** mode collapse (`G` finds a few outputs that reliably fool `D` and stops exploring the full data distribution), vanishing gradients when `D` becomes too strong too fast, and general instability from the adversarial min-max dynamic (no single loss that monotonically decreases). Mitigations: **WGAN** (Wasserstein loss + gradient penalty, gives a smoother, more meaningful training signal), spectral normalization, careful G/D learning-rate balancing, label smoothing on real labels.

**Use when:** you need the sharpest possible sample quality and don't need an explicit likelihood or easy latent-space interpolation (classic use: image super-resolution, style transfer, deepfakes/face synthesis, data augmentation for rare classes).

```python
# GAN skeleton (DCGAN-style) — training loop must alternate D and G updates manually
generator = keras.Sequential([
    layers.Input(shape=(100,)),
    layers.Dense(7*7*128, activation="relu"),
    layers.Reshape((7, 7, 128)),
    layers.Conv2DTranspose(64, 4, strides=2, padding="same", activation="relu"),
    layers.Conv2DTranspose(1, 4, strides=2, padding="same", activation="tanh"),
])
discriminator = keras.Sequential([
    layers.Input(shape=(28, 28, 1)),
    layers.Conv2D(64, 4, strides=2, padding="same"), layers.LeakyReLU(0.2),
    layers.Conv2D(128, 4, strides=2, padding="same"), layers.LeakyReLU(0.2),
    layers.Flatten(), layers.Dense(1, activation="sigmoid"),
])

bce = keras.losses.BinaryCrossentropy()
g_opt, d_opt = keras.optimizers.Adam(2e-4, 0.5), keras.optimizers.Adam(2e-4, 0.5)

@tf.function
def train_step(real_images, batch_size=64, noise_dim=100):
    noise = tf.random.normal([batch_size, noise_dim])
    with tf.GradientTape() as d_tape, tf.GradientTape() as g_tape:
        fake_images = generator(noise, training=True)
        real_pred = discriminator(real_images, training=True)
        fake_pred = discriminator(fake_images, training=True)
        d_loss = bce(tf.ones_like(real_pred), real_pred) + bce(tf.zeros_like(fake_pred), fake_pred)
        g_loss = bce(tf.ones_like(fake_pred), fake_pred)     # G wants D to say "real"
    d_opt.apply_gradients(zip(d_tape.gradient(d_loss, discriminator.trainable_variables), discriminator.trainable_variables))
    g_opt.apply_gradients(zip(g_tape.gradient(g_loss, generator.trainable_variables), generator.trainable_variables))
    return d_loss, g_loss
```

### Diffusion Models
Learn to reverse a gradual **noising process**. Forward process adds Gaussian noise over `T` steps until the data becomes pure noise; the model (usually a **U-Net**, often with attention blocks) is trained to predict the noise added at each step so it can be subtracted, and generation runs this denoising process backward from pure noise to a sample.

```
Forward:  x_t = √(ᾱ_t)·x_0 + √(1-ᾱ_t)·ε ,  ε ~ N(0,1)
Training objective: minimize  ||ε - ε_θ(x_t, t)||²     # simple MSE — much more stable than GAN's adversarial loss
```

**Why diffusion overtook GANs for image generation (Stable Diffusion, DALL-E, Imagen):** far more stable training (plain regression loss vs. adversarial min-max), better mode coverage (doesn't collapse), and higher fidelity at scale — at the cost of slow sampling (many denoising steps), which is what techniques like DDIM sampling, distillation, and latent-space diffusion (denoise in a compressed VAE latent, not raw pixels — Stable Diffusion's core trick) address.

**Generative model selection:**

| Model | Sample quality | Training stability | Latent space | Sampling speed |
|---|---|---|---|---|
| VAE | Blurry-ish | Very stable | Smooth, interpretable | Fast (1 pass) |
| GAN | Sharp | Unstable, needs tuning | Less structured | Fast (1 pass) |
| Diffusion | Best (SOTA) | Stable | N/A (iterative) | Slow (many steps) |

---

## 16. Transfer Learning & Fine-Tuning

Training from scratch requires huge labeled datasets. Transfer learning reuses a model pretrained on a large source dataset (e.g., ImageNet, or a text corpus for LLMs) as a starting point for a related target task.

**Feature extraction** — freeze the pretrained backbone entirely, train only a new head on top. Best when the target dataset is small and similar to the source domain.

**Fine-tuning** — unfreeze some/all backbone layers and continue training at a **much lower learning rate** than training from scratch, so pretrained weights aren't destroyed by large early gradient updates. Best when the target dataset is larger or more different from the source domain.

**Common practical recipe:** freeze the backbone, train the head for a few epochs until it stabilizes, then unfreeze the top few backbone layers (or all of it) and fine-tune everything end-to-end with a small LR.

```python
base_model = keras.applications.EfficientNetV2B0(
    include_top=False, weights="imagenet", input_shape=(224, 224, 3)
)
base_model.trainable = False   # Phase 1: feature extraction

inputs = keras.Input(shape=(224, 224, 3))
x = keras.applications.efficientnet_v2.preprocess_input(inputs)
x = base_model(x, training=False)
x = layers.GlobalAveragePooling2D()(x)
x = layers.Dropout(0.3)(x)
outputs = layers.Dense(num_classes, activation="softmax")(x)
model = keras.Model(inputs, outputs)

model.compile(optimizer=keras.optimizers.Adam(1e-3), loss="sparse_categorical_crossentropy", metrics=["accuracy"])
model.fit(train_ds, validation_data=val_ds, epochs=5)

# Phase 2: fine-tune the top of the backbone
base_model.trainable = True
for layer in base_model.layers[:-30]:
    layer.trainable = False    # keep early, general-purpose layers frozen

model.compile(optimizer=keras.optimizers.Adam(1e-5), loss="sparse_categorical_crossentropy", metrics=["accuracy"])
model.fit(train_ds, validation_data=val_ds, epochs=5)
```

**Why the LR drops for fine-tuning:** pretrained weights already encode useful features; a large LR would apply large gradient updates that overwrite that knowledge before the new head has learned anything useful to backpropagate ("catastrophic forgetting").

---

## 17. Data Pipelines & Augmentation

**`tf.data` pipeline** — the standard, performant way to feed data to a Keras model, supporting streaming from disk, shuffling, batching, prefetching (overlaps data loading with GPU compute).

```python
def preprocess(image, label):
    image = tf.image.resize(image, (224, 224))
    image = tf.cast(image, tf.float32) / 255.0
    return image, label

train_ds = (
    tf.data.Dataset.from_tensor_slices((file_paths, labels))
    .map(load_and_decode, num_parallel_calls=tf.data.AUTOTUNE)
    .map(preprocess, num_parallel_calls=tf.data.AUTOTUNE)
    .shuffle(1000)
    .batch(32)
    .prefetch(tf.data.AUTOTUNE)     # overlap CPU preprocessing with GPU training
)
```

**Data augmentation** — synthetically expands training data by applying label-preserving transformations, acting as a strong regularizer (particularly important when data is limited).

- **Vision:** random flip, rotation, crop, color jitter, cutout/random erasing, MixUp (linearly blend two images and their labels), CutMix (paste a patch from one image onto another, mix labels proportionally to patch area).
- **NLP:** synonym replacement, back-translation, random token masking/deletion.
- **Audio:** time/frequency masking (SpecAugment), pitch shift, noise injection.

```python
data_augmentation = keras.Sequential([
    layers.RandomFlip("horizontal"),
    layers.RandomRotation(0.1),
    layers.RandomZoom(0.1),
    layers.RandomContrast(0.1),
])

inputs = keras.Input(shape=(224, 224, 3))
x = data_augmentation(inputs)     # active only during training (Keras handles this automatically)
x = base_model(x)
```

---

## 18. Hyperparameter Tuning

Key hyperparameters, roughly ordered by typical impact: **learning rate** > architecture/model size > batch size > regularization strength (dropout rate, weight decay) > optimizer choice > LR schedule details.

**Search strategies:**
- **Grid search** — exhaustive over a fixed grid. Simple but scales exponentially with hyperparameter count — impractical beyond 2-3 dimensions.
- **Random search** — sample randomly from each hyperparameter's distribution. Surprisingly outperforms grid search in high dimensions because it doesn't waste trials on unimportant dimensions (Bergstra & Bengio, 2012).
- **Bayesian optimization** — builds a probabilistic surrogate model (commonly a Gaussian Process) of the objective as a function of hyperparameters, and picks the next trial to maximize expected improvement. Much more sample-efficient than random search for expensive-to-train models.
- **Hyperband / ASHA** — allocate a small compute budget to many configurations, then progressively kill the worst performers and allocate more budget to survivors ("successive halving"). Efficient for large search spaces where most configs are quickly identifiable as bad.

```python
import keras_tuner as kt

def build_model(hp):
    model = keras.Sequential()
    model.add(layers.Input(shape=(28, 28, 1)))
    for i in range(hp.Int("num_layers", 1, 3)):
        model.add(layers.Conv2D(hp.Choice(f"filters_{i}", [32, 64, 128]), 3, activation="relu"))
        model.add(layers.MaxPooling2D())
    model.add(layers.Flatten())
    model.add(layers.Dropout(hp.Float("dropout", 0.1, 0.5, step=0.1)))
    model.add(layers.Dense(10, activation="softmax"))
    lr = hp.Float("lr", 1e-4, 1e-2, sampling="log")
    model.compile(optimizer=keras.optimizers.Adam(lr), loss="sparse_categorical_crossentropy", metrics=["accuracy"])
    return model

tuner = kt.Hyperband(build_model, objective="val_accuracy", max_epochs=20, factor=3)
tuner.search(X_train, y_train, validation_split=0.2, epochs=20)
best_model = tuner.get_best_models(1)[0]
```

**Practical tip:** always tune learning rate first (a simple LR range test — ramp LR exponentially over a few hundred steps and watch when loss starts diverging — gives a good starting point) before spending compute tuning architecture.

---

## 19. Evaluation Metrics

Choice of metric should match the actual cost of different error types — accuracy alone is often misleading.

**Classification:**
- **Accuracy** — `correct/total`. Misleading on imbalanced data (predicting the majority class always can give high "accuracy").
- **Precision** — `TP/(TP+FP)` — of predicted positives, how many were correct. Prioritize when false positives are costly (spam filter flagging real email).
- **Recall** — `TP/(TP+FN)` — of actual positives, how many were caught. Prioritize when false negatives are costly (cancer screening missing a real case).
- **F1** — harmonic mean of precision & recall; use when you need a single balance-of-both metric on imbalanced data.
- **AUC-ROC** — probability the model ranks a random positive above a random negative, across all thresholds; threshold-independent view of separability.
- **Confusion matrix** — the full breakdown; always worth inspecting directly, not just summary metrics.

**Regression:** MSE/RMSE (penalize large errors), MAE (robust, interpretable in original units), R² (variance explained relative to a naive mean predictor).

**Object detection/segmentation:** **IoU** (Intersection over Union — overlap between predicted and ground-truth boxes/masks), **mAP** (mean Average Precision across classes and IoU thresholds).

**NLP generation:** **Perplexity** (`exp(cross-entropy loss)` — how "surprised" the model is by held-out text; lower is better), **BLEU/ROUGE** (n-gram overlap with reference text — machine translation/summarization), increasingly replaced by embedding-based or LLM-judge metrics for open-ended generation quality.

```python
model.compile(
    optimizer="adam",
    loss="binary_crossentropy",
    metrics=["accuracy", keras.metrics.Precision(), keras.metrics.Recall(), keras.metrics.AUC()],
)
```

---

## 20. Bias-Variance, Overfitting & Debugging Training

**Bias-variance tradeoff:**
- **High bias (underfitting)** — model too simple to capture the underlying pattern; poor performance on *both* train and validation sets. Fix: bigger model, more features, train longer, reduce regularization.
- **High variance (overfitting)** — model memorizes training data noise; great train performance, poor validation performance. Fix: more data, data augmentation, regularization (dropout, weight decay, early stopping), smaller model, simplify architecture.

**Diagnostic workflow when a model isn't training well:**
1. **Loss is NaN/exploding** → learning rate too high, missing gradient clipping, bad input normalization, or a numerically unstable loss (unclipped log in custom loss).
2. **Loss barely decreases from the start** → LR too low, dead ReLUs (check activation stats), bad initialization, vanishing gradients in a very deep net without normalization/residuals, or a bug in the loss/label pipeline (verify with a tiny overfit test below).
3. **Train loss goes down, val loss goes up (diverges)** → classic overfitting — apply §7 regularization or get more data.
4. **Both train and val loss plateau high** → underfitting — increase model capacity, train longer, check the LR isn't too conservative.
5. **Sanity check: can the model overfit a tiny subset (e.g., 10 examples)?** If it *can't* drive that loss near zero, there's a bug (wrong loss, frozen layers, shuffled labels, data leakage in preprocessing) — not a capacity or regularization problem. This is one of the most reliable debugging steps in deep learning and should be step one before any hyperparameter tuning.
6. **Data leakage check** — make sure preprocessing statistics (normalization mean/std, tokenizer vocab) are fit only on train data, not on validation/test, and that no validation examples leaked into training (common with time-series or duplicated records).

```python
history = model.fit(X_train, y_train, validation_data=(X_val, y_val), epochs=50)

import matplotlib.pyplot as plt
plt.plot(history.history["loss"], label="train")
plt.plot(history.history["val_loss"], label="val")
plt.legend(); plt.show()
```

---

## 21. Distributed & Mixed-Precision Training

**Data parallelism** — replicate the full model on each device (GPU/TPU core), split each batch across devices, compute gradients locally, then average gradients across devices before applying the update (synchronous, via all-reduce). The standard approach for most training jobs.

```python
strategy = tf.distribute.MirroredStrategy()          # single machine, multiple GPUs
with strategy.scope():
    model = build_model()
    model.compile(optimizer="adam", loss="sparse_categorical_crossentropy", metrics=["accuracy"])
model.fit(train_ds, epochs=10)

# Multi-worker (multi-machine):
# strategy = tf.distribute.MultiWorkerMirroredStrategy()
# TPU:
# resolver = tf.distribute.cluster_resolver.TPUClusterResolver()
# strategy = tf.distribute.TPUStrategy(resolver)
```

**Model parallelism** — split the model itself across devices (different layers/tensor shards on different devices) when a single model doesn't fit in one device's memory — necessary for very large models (LLM training). Combines with data parallelism in large-scale training (3D parallelism: data + tensor + pipeline).

**Mixed-precision training** — perform most compute in `float16`/`bfloat16` (2x throughput and memory savings on modern GPU/TPU tensor cores) while keeping a `float32` master copy of weights and using **loss scaling** (multiply the loss by a large constant before backprop, then unscale gradients) to prevent small gradient values from underflowing to zero in float16's limited range.

```python
keras.mixed_precision.set_global_policy("mixed_float16")
# Keras automatically keeps the final Dense/softmax layer in float32 for numerical stability,
# and the optimizer (if using model.fit) handles loss scaling automatically.
```

**Gradient accumulation** — simulate a larger effective batch size than fits in memory by accumulating gradients over several mini-batches before applying an optimizer step. Useful for large models on limited GPU memory.

---

## 22. TensorFlow/Keras Practical Patterns

**Three ways to build a model — pick based on flexibility needs:**

```python
# 1. Sequential — simple linear stack, least flexible
model = keras.Sequential([layers.Dense(64, activation="relu"), layers.Dense(10)])

# 2. Functional API — supports multi-input/output, branching, skip connections (most common for real projects)
inputs = keras.Input(shape=(32,))
x = layers.Dense(64, activation="relu")(inputs)
outputs = layers.Dense(10, activation="softmax")(x)
model = keras.Model(inputs, outputs)

# 3. Subclassing — full control (custom forward logic, dynamic architectures), most flexible, least "free" tooling
class MyModel(keras.Model):
    def __init__(self):
        super().__init__()
        self.dense1 = layers.Dense(64, activation="relu")
        self.dense2 = layers.Dense(10, activation="softmax")
    def call(self, inputs, training=False):
        x = self.dense1(inputs)
        return self.dense2(x)
```

**Callbacks (used constantly in real training loops):**
```python
callbacks = [
    keras.callbacks.ModelCheckpoint("best_model.keras", save_best_only=True, monitor="val_loss"),
    keras.callbacks.EarlyStopping(patience=10, restore_best_weights=True),
    keras.callbacks.ReduceLROnPlateau(factor=0.5, patience=5),
    keras.callbacks.TensorBoard(log_dir="./logs"),
]
model.fit(train_ds, validation_data=val_ds, epochs=100, callbacks=callbacks)
```

**Custom training loop** (needed for GANs, RL, or any training logic `model.fit` can't express — see §15's GAN example, or the manual `train_step` in §4):
- Use `tf.GradientTape()` to record operations for automatic differentiation.
- Wrap the step function in `@tf.function` to compile it into a graph for a large speedup over eager execution.

**Saving/loading:**
```python
model.save("model.keras")                       # full model: architecture + weights + optimizer state
loaded = keras.models.load_model("model.keras")

model.save_weights("weights.h5")                 # weights only — need to rebuild architecture to reload
```

**Deployment paths:**
```python
# TensorFlow Lite — mobile/edge inference
converter = tf.lite.TFLiteConverter.from_keras_model(model)
converter.optimizations = [tf.lite.Optimize.DEFAULT]   # post-training quantization
tflite_model = converter.convert()

# TensorFlow Serving — production REST/gRPC serving
model.export("saved_model_dir")   # SavedModel format, served via `tensorflow_model_server`

# TensorFlow.js — browser inference
# tensorflowjs_converter --input_format=keras model.keras web_model/
```

**Quantization** (post-training or quantization-aware training) — represent weights/activations in int8 instead of float32, cutting model size ~4x and speeding up inference on supporting hardware, with a small accuracy cost. Standard for edge deployment.

---

## 23. "Which Algorithm/Architecture Do I Use?" Cheat Sheet

| Task | Go-to architecture | Notes |
|---|---|---|
| Image classification | CNN (EfficientNet/ResNet, transfer learning) or ViT if you have huge data | Start with a pretrained backbone, fine-tune (§16) |
| Object detection | YOLO (real-time) / Faster R-CNN (accuracy-focused) / DETR (Transformer-based) | Anchor-based (YOLO/Faster R-CNN) vs. set-prediction (DETR) tradeoffs |
| Image segmentation | U-Net / DeepLab / Mask R-CNN | U-Net's skip connections preserve spatial detail |
| Tabular data | Gradient boosted trees (XGBoost/LightGBM) usually beats deep learning | Deep learning shines with huge data, images/text/sequences, or when you need learned embeddings for categorical features (entity embeddings) |
| Text classification | Fine-tuned Transformer encoder (BERT-family) | Small-data/latency-constrained → TF-IDF + linear model or a small CNN/LSTM can be enough |
| Text generation | Transformer decoder (GPT-family), fine-tune or prompt a pretrained LLM | See `ai/genai_interview_guide.md` |
| Time-series forecasting | LSTM/GRU (nonlinear, sequential), Temporal CNN, or Transformer for long-range; classical (ARIMA/Prophet) for simple/short series | Deep learning wins with multiple correlated series and enough history |
| Recommendation systems | Two-tower embedding models, deep & wide networks, GNNs for graph-structured interactions | Cold-start problems often need hybrid content-based fallbacks |
| Anomaly detection | Autoencoder reconstruction error, isolation forests (non-deep baseline) | AE flags high reconstruction error as anomalous |
| Face/similarity/retrieval | CNN + triplet/contrastive loss → embedding space, nearest-neighbor search | Same pattern underlies modern semantic search & RAG retrieval |
| Image generation | Diffusion model (best quality) or GAN (fast sampling, sharper but less stable) | VAE if you need a smooth, interpretable latent space |
| Speech recognition | CNN/Conformer (CNN+Transformer hybrid) encoder + CTC or attention decoder | Conformer is the modern standard (Whisper-style) |
| Graph-structured data | Graph Neural Network (GCN/GAT/GraphSAGE) | When relationships between entities matter more than grid structure |
| Reinforcement learning | Policy gradient / PPO (continuous control), DQN-family (discrete actions) | Different training paradigm entirely — reward-based, not supervised |
| Small dataset, any modality | Transfer learning from a pretrained model — almost always beats training from scratch | Deep learning generally needs either lots of data or a pretrained starting point |

**General decision heuristics:**
- **Structured/tabular data with <100K rows** → try gradient boosting before deep learning; it usually wins and is far cheaper to train/tune.
- **Any modality with a strong pretrained model available** → fine-tune it. Training from scratch is rarely the right default in 2026.
- **Need interpretability/regulatory auditability** → prefer simpler models (linear/tree-based) or restrict deep learning to feature extraction feeding an interpretable head.
- **Latency-critical edge deployment** → MobileNet/EfficientNet-Lite class CNNs, quantization, knowledge distillation into a smaller student model.
- **Sequence length is very long (thousands+ tokens) and compute is limited** → consider efficient attention variants (sliding window, linear attention) or hybrid CNN/RNN preprocessing before a Transformer, rather than full O(n²) attention.

---

**Where to go next:** `ai/genai_interview_guide.md` covers Transformers, attention, and LLM-specific topics (RAG, fine-tuning methods like LoRA/QLoRA, RLHF/DPO, quantization, serving) in much greater depth — read it alongside this file for the full modern deep learning + GenAI picture.
