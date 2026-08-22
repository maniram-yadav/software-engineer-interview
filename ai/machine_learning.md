# Machine Learning — In-Depth Reference

A practical, theory-backed guide to ML algorithms, when to use them, how they're trained/optimized, and how to implement them with `scikit-learn`.

---

## 1. Foundations

### 1.1 Types of Learning
- **Supervised**: labeled data `(X, y)`. Regression (continuous `y`) or Classification (discrete `y`).
- **Unsupervised**: no labels. Clustering, dimensionality reduction, density estimation.
- **Semi-supervised**: small labeled + large unlabeled set.
- **Self-supervised**: labels derived from the data itself (e.g., predict masked tokens/pixels).
- **Reinforcement Learning**: agent learns a policy via reward signals (not covered by sklearn — see `gymnasium` + `stable-baselines3`).

### 1.2 Bias–Variance Tradeoff
Expected test error decomposes as:

```
Error(x) = Bias² + Variance + Irreducible Noise
```

- **High bias (underfitting)**: model too simple, misses patterns. Train and test error both high.
- **High variance (overfitting)**: model too complex, memorizes noise. Train error low, test error high.
- **Fix for high bias**: more features, more complex model, less regularization, train longer.
- **Fix for high variance**: more data, regularization, simpler model, feature selection, ensembling (bagging), early stopping, dropout.

Diagnose with **learning curves** (`sklearn.model_selection.learning_curve`) and **validation curves** (`validation_curve`).

### 1.3 Train / Validation / Test Discipline
- **Train set**: fit parameters.
- **Validation set**: tune hyperparameters, select model.
- **Test set**: touched exactly once, at the end, for an unbiased estimate.
- **k-Fold Cross-Validation**: split data into k folds, train on k-1, validate on 1, rotate. Reduces variance of the performance estimate vs. a single split.
- **Stratified k-Fold**: preserves class proportions per fold — always use for classification with imbalance.
- **TimeSeriesSplit**: for temporal data, never shuffle; train on past, validate on future.
- **Nested CV**: outer loop for unbiased performance estimate, inner loop for hyperparameter search — avoids optimistic bias from tuning on the same folds you report on.

```python
from sklearn.model_selection import (
    train_test_split, KFold, StratifiedKFold, cross_val_score, TimeSeriesSplit
)

X_train, X_test, y_train, y_test = train_test_split(
    X, y, test_size=0.2, stratify=y, random_state=42
)

skf = StratifiedKFold(n_splits=5, shuffle=True, random_state=42)
scores = cross_val_score(model, X_train, y_train, cv=skf, scoring="f1_macro")
print(scores.mean(), scores.std())
```

### 1.4 Data Leakage (the #1 silent killer of real-world ML)
- Never fit scalers/encoders/imputers on the full dataset before splitting — fit on train, transform on val/test.
- Use `Pipeline` so preprocessing is refit correctly inside each CV fold.
- Watch for target leakage: features that encode the label indirectly (e.g., "cancellation_date" predicting "will_cancel").
- Group-aware splitting (`GroupKFold`) when multiple rows belong to the same entity (e.g., same user/patient) to prevent identity leakage across folds.

---

## 2. Data Preprocessing & Feature Engineering

```python
from sklearn.compose import ColumnTransformer
from sklearn.pipeline import Pipeline
from sklearn.preprocessing import StandardScaler, OneHotEncoder, OrdinalEncoder
from sklearn.impute import SimpleImputer, KNNImputer

numeric_features = ["age", "income"]
categorical_features = ["city", "gender"]

numeric_pipe = Pipeline([
    ("imputer", SimpleImputer(strategy="median")),
    ("scaler", StandardScaler()),
])

categorical_pipe = Pipeline([
    ("imputer", SimpleImputer(strategy="most_frequent")),
    ("onehot", OneHotEncoder(handle_unknown="ignore")),
])

preprocess = ColumnTransformer([
    ("num", numeric_pipe, numeric_features),
    ("cat", categorical_pipe, categorical_features),
])
```

**Scaling — when it matters:**
- Distance/gradient-based models (KNN, SVM, logistic/linear regression with regularization, neural nets, PCA, k-means) **need** scaling.
- Tree-based models (Decision Tree, Random Forest, Gradient Boosting) are **scale-invariant** — skip it.
- `StandardScaler` (zero mean, unit variance) — default choice, assumes roughly Gaussian.
- `MinMaxScaler` — bounds to [0,1], good for neural nets / when distribution isn't Gaussian.
- `RobustScaler` — uses median/IQR, robust to outliers.

**Encoding categoricals:**
- `OneHotEncoder` — nominal, low cardinality.
- `OrdinalEncoder` — ordinal (has natural order).
- Target/mean encoding — high cardinality, but **must** be fit only on train folds (leakage risk) — use `category_encoders.TargetEncoder` with CV.
- Tree models can handle high-cardinality ordinal-encoded categoricals reasonably well; linear models cannot.

**Imbalanced classes:**
```python
from imblearn.over_sampling import SMOTE
from imblearn.pipeline import Pipeline as ImbPipeline

pipe = ImbPipeline([
    ("preprocess", preprocess),
    ("smote", SMOTE(random_state=42)),
    ("clf", LogisticRegression(class_weight="balanced")),
])
```
Alternatives: `class_weight="balanced"`, undersampling majority class, threshold tuning on predicted probabilities, or metrics like PR-AUC/F1 instead of accuracy.

---

## 3. Linear Models

### 3.1 Linear Regression
**Model**: `ŷ = Xw + b`. Minimizes **Mean Squared Error (MSE)**:
```
J(w) = (1/n) Σ (y_i - ŷ_i)²
```
**Closed-form (Normal Equation)**: `w = (XᵀX)⁻¹ Xᵀy` — exact but O(d³), unstable when features are collinear or d is large.
**Gradient Descent**: iteratively `w ← w - η ∇J(w)` — scales to large d/n.

Assumptions: linearity, independence of errors, homoscedasticity (constant error variance), no severe multicollinearity, errors ~ Normal (for inference, not prediction).

```python
from sklearn.linear_model import LinearRegression
model = LinearRegression().fit(X_train, y_train)
```

### 3.2 Regularized Linear Models
Add a penalty to reduce variance / handle multicollinearity / do feature selection.

| Model | Penalty | Effect |
|---|---|---|
| **Ridge** | `α Σ w_j²` (L2) | Shrinks weights smoothly toward 0, keeps all features, handles multicollinearity well |
| **Lasso** | `α Σ \|w_j\|` (L1) | Drives some weights to exactly 0 → automatic feature selection, sparse models |
| **ElasticNet** | `α(ρ‖w‖₁ + (1-ρ)‖w‖₂²)` | Combines both, good when features are correlated and you still want sparsity |

```python
from sklearn.linear_model import Ridge, Lasso, ElasticNet
from sklearn.linear_model import RidgeCV, LassoCV  # built-in CV for alpha

ridge = RidgeCV(alphas=[0.01, 0.1, 1.0, 10.0]).fit(X_train, y_train)
lasso = LassoCV(cv=5).fit(X_train, y_train)
```
**When to use**: Ridge when all features are plausibly useful and correlated; Lasso when you suspect many features are irrelevant and want a sparse, interpretable model; ElasticNet as a safe default between the two.

### 3.3 Logistic Regression (classification, not regression)
Models `P(y=1|x) = σ(wᵀx + b)` where `σ(z) = 1/(1+e⁻ᶻ)`. Trained via **Maximum Likelihood** → minimizing **log-loss (cross-entropy)**:
```
J(w) = -(1/n) Σ [y_i log(p_i) + (1-y_i) log(1-p_i)] + regularization
```
No closed form — solved via gradient-based solvers (`lbfgs`, `saga`, `liblinear`, `newton-cg`). Decision boundary is linear in feature space (can be made non-linear via polynomial/kernel features).

```python
from sklearn.linear_model import LogisticRegression
clf = LogisticRegression(C=1.0, penalty="l2", solver="lbfgs", max_iter=1000)
clf.fit(X_train, y_train)
proba = clf.predict_proba(X_test)[:, 1]
```
Note: sklearn's `C` is the **inverse** of regularization strength (`C = 1/α`) — smaller `C` = stronger regularization.

**When to use**: baseline for any binary/multiclass problem, when interpretability (odds ratios via `exp(coef_)`) matters, when the problem is roughly linearly separable, low-latency inference needs.

---

## 4. Optimization Techniques (the training engine behind most models)

### 4.1 Gradient Descent Variants
- **Batch GD**: uses full dataset per step. Stable convergence, slow/memory-heavy for large n.
- **Stochastic GD (SGD)**: one sample per step. Noisy but fast, can escape shallow local minima, needs learning-rate decay.
- **Mini-batch GD**: the practical default (32–512 samples/step) — balances stability and speed, GPU-friendly.

### 4.2 Adaptive / Momentum-Based Optimizers (used by `MLPClassifier`, and everywhere in deep learning)
- **Momentum**: `v ← βv + ∇J(w); w ← w - ηv` — accelerates in consistent gradient directions, dampens oscillation.
- **Nesterov Momentum**: looks ahead before computing the gradient — corrects overshoot earlier.
- **AdaGrad**: per-parameter learning rate, scaled inversely by cumulative squared gradients — good for sparse features but learning rate shrinks too aggressively over time.
- **RMSprop**: fixes AdaGrad's decay with an exponential moving average of squared gradients.
- **Adam**: combines Momentum (1st moment) + RMSprop (2nd moment), with bias correction — the default choice for most deep nets due to fast, robust convergence.

```python
from sklearn.neural_network import MLPClassifier
mlp = MLPClassifier(
    hidden_layer_sizes=(64, 32), activation="relu",
    solver="adam", alpha=1e-4, learning_rate_init=1e-3,
    early_stopping=True, max_iter=500
)
mlp.fit(X_train, y_train)
```

### 4.3 Second-Order Methods
- **Newton's Method**: uses the Hessian (curvature) for faster convergence per step, but O(d²)–O(d³) per iteration — impractical for high-dimensional models.
- **L-BFGS**: quasi-Newton, approximates the Hessian with limited memory — sklearn's default solver for `LogisticRegression`/small `MLPClassifier` because it converges in few iterations on small-to-medium data.

### 4.4 Convex vs Non-Convex Optimization
- Linear/Logistic Regression, SVM (with convex loss) → **convex** loss surface → global minimum guaranteed by gradient descent.
- Neural networks, deep trees → **non-convex** → gradient descent finds *a* local minimum (empirically usually good enough); initialization, learning rate schedule, and batch size matter a lot.

### 4.5 Learning Rate Scheduling
- Constant, step decay, exponential decay, cosine annealing, warm restarts, `ReduceLROnPlateau`.
- Too high → divergence/oscillation. Too low → painfully slow, may get stuck.
- **Learning rate finder** (increase LR exponentially over a mini run, plot loss) is a common practical trick outside sklearn (e.g., fastai, PyTorch).

---

## 5. Regularization (Generalization Techniques)

- **L1/L2 weight penalties** — see §3.2.
- **Early stopping** — stop training when validation loss stops improving (`early_stopping=True` in `MLPClassifier`, `GradientBoostingClassifier` with `n_iter_no_change`).
- **Dropout** — randomly zero neurons during training (deep learning; not in sklearn's MLP, available in Keras/PyTorch).
- **Data augmentation** — synthetically expand training data (images/text; SMOTE for tabular imbalance).
- **Ensembling** — averaging reduces variance (see §7).
- **Max depth / min samples leaf / min samples split** — regularization for trees.
- **Batch normalization** — deep learning, stabilizes/regularizes training by normalizing layer inputs.

---

## 6. Instance-Based & Probabilistic Models

### 6.1 k-Nearest Neighbors (KNN)
**Non-parametric, lazy learner** — no real "training," just stores data. Prediction = majority vote (classification) or average (regression) of the `k` closest points (typically Euclidean/Minkowski distance).

```python
from sklearn.neighbors import KNeighborsClassifier
knn = KNeighborsClassifier(n_neighbors=5, weights="distance", metric="minkowski", p=2)
knn.fit(X_train, y_train)
```
- **k too small** → high variance (overfits to noise). **k too large** → high bias (oversmooths, approaches majority class).
- Requires feature scaling. Suffers from the **curse of dimensionality** — distances become less meaningful in high-d.
- **When to use**: small-to-medium datasets, low-dimensional, when decision boundary is highly irregular and you don't need a compact model. Poor for high-d, large n (slow at inference — mitigate with KD-Tree/Ball-Tree, or `algorithm="auto"`).

### 6.2 Naive Bayes
Applies Bayes' theorem with a (naive) conditional independence assumption between features given the class:
```
P(y|x₁,...,x_d) ∝ P(y) Π P(x_i|y)
```
Training = just estimating `P(y)` and `P(x_i|y)` from frequency/likelihood — extremely fast, no iterative optimization.

```python
from sklearn.naive_bayes import GaussianNB, MultinomialNB, BernoulliNB
gnb = GaussianNB().fit(X_train, y_train)          # continuous features
mnb = MultinomialNB().fit(X_train_counts, y_train)  # word counts / TF-IDF (text)
bnb = BernoulliNB().fit(X_train_binary, y_train)    # binary features
```
**When to use**: text classification / spam filtering (`MultinomialNB` on bag-of-words/TF-IDF is a strong, fast baseline), very high-dimensional sparse data, when you need a probabilistic baseline fast, small training sets.

### 6.3 Support Vector Machines (SVM)
**Idea**: find the hyperplane that maximizes the margin between classes. Only "support vectors" (points near/on the margin) matter for the boundary.

**Hard margin** (linearly separable, no errors allowed) vs **Soft margin** (allows misclassification, controlled by `C`):
```
min (1/2)‖w‖² + C Σ ξ_i     s.t. y_i(wᵀx_i + b) ≥ 1 - ξ_i
```
- **C** (inverse regularization): small C → wider margin, more tolerant of errors (more bias, less variance). Large C → tries to classify every point correctly (less bias, more variance, risk of overfitting).

**Kernel trick**: maps data into higher-dimensional space implicitly (via a kernel function) without ever computing the transform explicitly, enabling non-linear boundaries.
- Linear kernel — linearly separable data, high-d sparse data (text).
- RBF (Gaussian) kernel — default general-purpose non-linear kernel. `gamma` controls how far the influence of a single point reaches (high gamma = tight, wiggly boundary → overfit risk; low gamma = smooth boundary → underfit risk).
- Polynomial kernel — when interactions of a known degree matter.

```python
from sklearn.svm import SVC
svm = SVC(kernel="rbf", C=1.0, gamma="scale", probability=True)
svm.fit(X_train, y_train)
```
Trained via **quadratic programming** (dual formulation with Lagrange multipliers) — SMO (Sequential Minimal Optimization) algorithm under the hood in `libsvm`.

**When to use**: small-to-medium datasets, high-dimensional data (text, bio), clear margin of separation expected, when you need a robust, well-regularized classifier and can afford `O(n²)`–`O(n³)` training cost. Not ideal for very large n (use `LinearSVC`/`SGDClassifier` instead — they scale linearly).

---

## 7. Tree-Based Models & Ensembles

### 7.1 Decision Trees
Recursively splits data on the feature/threshold that most reduces **impurity**:
- **Gini impurity**: `1 - Σ p_k²` (default for `DecisionTreeClassifier`, faster).
- **Entropy / Information Gain**: `-Σ p_k log₂(p_k)`; gain = parent entropy − weighted child entropy.
- **MSE** for regression trees (`DecisionTreeRegressor`).

Greedy, recursive binary splitting (CART algorithm) — no global optimum guaranteed, but computationally efficient.

```python
from sklearn.tree import DecisionTreeClassifier, plot_tree
tree = DecisionTreeClassifier(max_depth=5, min_samples_leaf=10, criterion="gini")
tree.fit(X_train, y_train)
```
- Prone to overfitting if unconstrained (grows until pure leaves) — control via `max_depth`, `min_samples_split`, `min_samples_leaf`, `ccp_alpha` (cost-complexity pruning).
- No scaling needed. Handles non-linear relationships and feature interactions natively. Highly interpretable (can visualize). High variance — small data changes → very different tree (this is exactly what ensembling fixes).

### 7.2 Bagging & Random Forest
**Bagging (Bootstrap Aggregating)**: train many models on bootstrap resamples (sampling with replacement) of the data, average their predictions → reduces **variance** without increasing bias.

**Random Forest** = Bagging of decision trees + random feature subsampling at each split (decorrelates trees further).

```python
from sklearn.ensemble import RandomForestClassifier
rf = RandomForestClassifier(
    n_estimators=300, max_depth=None, max_features="sqrt",
    min_samples_leaf=2, n_jobs=-1, random_state=42
)
rf.fit(X_train, y_train)
importances = rf.feature_importances_
```
**When to use**: strong general-purpose tabular baseline, robust to outliers/scaling, handles mixed feature types, gives free feature importance, parallelizable (independent trees), rarely needs heavy tuning. Weaker than boosting on very structured/clean tabular data, larger memory footprint, less interpretable than a single tree.

### 7.3 Boosting
Builds models **sequentially**, each new model focusing on the errors of the previous ensemble — reduces **bias** (and can reduce variance too with shrinkage).

**AdaBoost**: reweights misclassified samples higher each round; combines weak learners (usually depth-1 "stumps") via weighted majority vote.
```python
from sklearn.ensemble import AdaBoostClassifier
ada = AdaBoostClassifier(n_estimators=200, learning_rate=1.0)
```

**Gradient Boosting**: each new tree fits the **negative gradient (residual)** of the loss function w.r.t. current predictions — generalizes boosting to any differentiable loss.
```
F_m(x) = F_{m-1}(x) + η · h_m(x)     where h_m fits the pseudo-residuals
```
```python
from sklearn.ensemble import GradientBoostingClassifier, HistGradientBoostingClassifier
gbc = HistGradientBoostingClassifier(   # fast, histogram-based — prefer this in sklearn
    max_iter=300, learning_rate=0.05, max_depth=6,
    l2_regularization=1.0, early_stopping=True
)
gbc.fit(X_train, y_train)
```
Industry-standard external libraries (drop-in sklearn-compatible API): **XGBoost**, **LightGBM**, **CatBoost** — faster, more regularization knobs, native categorical support (CatBoost), usually win Kaggle tabular competitions.

**Key hyperparameters (bias/variance dials) for boosting**:
- `learning_rate` (shrinkage) — lower = needs more `n_estimators`/`max_iter` but generalizes better.
- `max_depth`/`num_leaves` — controls per-tree complexity.
- `subsample` (stochastic gradient boosting: row subsampling) and `colsample_bytree` (feature subsampling) — add randomness, reduce overfitting, speed up training.
- `n_estimators` with early stopping on a validation set — the standard way to prevent overfitting.

**When to use**: the go-to for tabular data competitions/production when you need top accuracy and can afford tuning; generally beats Random Forest on structured data with enough tuning time; more prone to overfitting than RF if not regularized/early-stopped.

### 7.4 Stacking
Train diverse base models, then train a **meta-model** on their out-of-fold predictions to learn how to best combine them.
```python
from sklearn.ensemble import StackingClassifier
stack = StackingClassifier(
    estimators=[("rf", RandomForestClassifier()), ("svc", SVC(probability=True))],
    final_estimator=LogisticRegression(),
    cv=5
)
```
**When to use**: squeezing out final performance gains (competitions), when base models have different, complementary error patterns.

### Random Forest vs Gradient Boosting — quick decision rule
| | Random Forest | Gradient Boosting |
|---|---|---|
| Reduces | Variance | Bias (and variance with shrinkage) |
| Trains | Parallel, independent trees | Sequential, dependent trees |
| Tuning effort | Low | Higher (learning_rate, depth, n_estimators interplay) |
| Overfitting risk | Lower | Higher if untuned |
| Typical accuracy ceiling | Good | Usually higher with proper tuning |
| Speed to train | Fast (parallelizable) | Slower (sequential), though histogram/GPU variants are fast |

---

## 8. Unsupervised Learning

### 8.1 Clustering

**k-Means**: partitions data into k clusters by minimizing within-cluster sum of squares (**inertia**):
```
J = Σ_k Σ_{x∈C_k} ‖x - μ_k‖²
```
Trained via **Lloyd's algorithm** (EM-like): assign points to nearest centroid → recompute centroids → repeat until convergence. Sensitive to initialization (`k-means++` init mitigates this), needs scaling, assumes roughly spherical, similarly-sized clusters, requires choosing k upfront.

```python
from sklearn.cluster import KMeans
from sklearn.metrics import silhouette_score

km = KMeans(n_clusters=4, init="k-means++", n_init=10, random_state=42)
labels = km.fit_predict(X_scaled)
print(silhouette_score(X_scaled, labels))
```
**Choosing k**: elbow method (plot inertia vs k), silhouette score (higher = better-separated clusters), gap statistic.

**Hierarchical (Agglomerative) Clustering**: bottom-up merging of closest clusters (linkage: ward, complete, average, single) — produces a dendrogram, no need to pre-specify k, but O(n²)–O(n³), doesn't scale to large n.
```python
from sklearn.cluster import AgglomerativeClustering
agg = AgglomerativeClustering(n_clusters=4, linkage="ward")
```

**DBSCAN**: density-based — groups points with enough nearby neighbors (`min_samples` within `eps`), marks sparse points as noise/outliers. Finds arbitrary-shaped clusters, doesn't require k, robust to outliers, but sensitive to `eps`/`min_samples` and struggles with varying density.
```python
from sklearn.cluster import DBSCAN
db = DBSCAN(eps=0.5, min_samples=5).fit(X_scaled)
```

**Gaussian Mixture Models (GMM)**: soft/probabilistic clustering — assumes data is generated from a mixture of k Gaussians, trained via **Expectation-Maximization (EM)**: E-step computes responsibility (soft cluster assignment probabilities), M-step updates each Gaussian's mean/covariance/weight. Gives cluster membership probabilities, not hard labels — more flexible than k-means (allows elliptical clusters).
```python
from sklearn.mixture import GaussianMixture
gmm = GaussianMixture(n_components=4, covariance_type="full").fit(X_scaled)
```

**Which clustering algorithm?**
- Spherical, similar-sized clusters, know k → **k-Means** (fast, scalable).
- Arbitrary shapes, outliers present, unknown k → **DBSCAN**.
- Need dendrogram / hierarchy of clusters, small-medium n → **Agglomerative**.
- Overlapping/elliptical clusters, want probabilities → **GMM**.

### 8.2 Dimensionality Reduction

**PCA (Principal Component Analysis)**: finds orthogonal directions (principal components) that maximize explained variance. Computed via eigendecomposition of the covariance matrix (or SVD of the data matrix) — the top-k eigenvectors (by eigenvalue) are the new axes. Linear, unsupervised, deterministic.
```python
from sklearn.decomposition import PCA
pca = PCA(n_components=0.95)   # keep enough components for 95% variance
X_reduced = pca.fit_transform(X_scaled)
print(pca.explained_variance_ratio_)
```
**Use for**: noise reduction, visualization (2D/3D), speeding up downstream models, mitigating multicollinearity, compressing features before clustering/KNN.

**LDA (Linear Discriminant Analysis)**: supervised dimensionality reduction — finds directions that maximize **between-class** separation relative to **within-class** scatter. Also usable as a classifier.
```python
from sklearn.discriminant_analysis import LinearDiscriminantAnalysis
lda = LinearDiscriminantAnalysis(n_components=2).fit(X, y)
```

**t-SNE / UMAP**: non-linear, mainly for 2D/3D **visualization** of high-dimensional data (preserves local neighborhood structure). Not meant for general feature reduction before modeling — non-deterministic, doesn't preserve global distances well, no simple out-of-sample transform (t-SNE).
```python
from sklearn.manifold import TSNE
X_2d = TSNE(n_components=2, perplexity=30, random_state=42).fit_transform(X_scaled)
```

**PCA vs t-SNE/UMAP**: use PCA for preprocessing/speed/interpretability; use t-SNE/UMAP purely to *visually* explore cluster structure, never feed their output into a downstream supervised model as "features" for production use.

---

## 9. Model Evaluation

### 9.1 Classification Metrics
- **Accuracy** = `(TP+TN)/Total` — misleading under class imbalance.
- **Precision** = `TP/(TP+FP)` — of predicted positives, how many correct. Matters when false positives are costly (e.g., spam filter).
- **Recall (Sensitivity)** = `TP/(TP+FN)` — of actual positives, how many caught. Matters when false negatives are costly (e.g., cancer screening).
- **F1** = harmonic mean of precision & recall — good single metric under imbalance.
- **ROC-AUC** — probability the model ranks a random positive above a random negative; threshold-independent. Can be overly optimistic under heavy imbalance.
- **PR-AUC (Average Precision)** — better than ROC-AUC when positive class is rare.
- **Confusion Matrix** — the ground truth for all the above.
- **Log Loss** — penalizes confident wrong predictions, used when calibrated probabilities matter.

```python
from sklearn.metrics import classification_report, roc_auc_score, confusion_matrix
print(classification_report(y_test, y_pred))
print(roc_auc_score(y_test, y_proba))
```

### 9.2 Regression Metrics
- **MAE** — average absolute error, robust to outliers, same units as target.
- **MSE / RMSE** — penalizes large errors more (squared) — RMSE is in original units.
- **R²** — proportion of variance explained (1 = perfect, 0 = as good as predicting the mean, can go negative).
- **MAPE** — percentage error, intuitive but unstable near y≈0.

```python
from sklearn.metrics import mean_absolute_error, mean_squared_error, r2_score
rmse = mean_squared_error(y_test, y_pred, squared=False)
```

---

## 10. Hyperparameter Tuning

```python
from sklearn.model_selection import GridSearchCV, RandomizedSearchCV
from scipy.stats import randint, uniform

param_grid = {"n_estimators": [100, 300, 500], "max_depth": [3, 5, 8, None]}
grid = GridSearchCV(RandomForestClassifier(), param_grid, cv=5, scoring="f1_macro", n_jobs=-1)
grid.fit(X_train, y_train)

param_dist = {"n_estimators": randint(100, 800), "max_depth": randint(2, 12)}
rand_search = RandomizedSearchCV(
    RandomForestClassifier(), param_dist, n_iter=40, cv=5, scoring="f1_macro", n_jobs=-1, random_state=42
)
```
- **GridSearchCV**: exhaustive, guaranteed to find best combo in the grid, expensive — combinatorial blowup.
- **RandomizedSearchCV**: samples random combos, usually finds near-optimal solutions much faster, scales to more hyperparameters.
- **Bayesian Optimization** (`optuna`, `scikit-optimize`, `hyperopt`): models the objective function itself, intelligently picks the next point to try — most sample-efficient for expensive models.
- Always tune **inside** cross-validation and evaluate on a held-out test set never touched during search (nested CV, §1.3).

---

## 11. Putting It Together: Full Pipeline Example

```python
from sklearn.pipeline import Pipeline
from sklearn.ensemble import HistGradientBoostingClassifier
from sklearn.model_selection import RandomizedSearchCV, StratifiedKFold
from sklearn.metrics import roc_auc_score

full_pipe = Pipeline([
    ("preprocess", preprocess),       # ColumnTransformer from §2
    ("clf", HistGradientBoostingClassifier(random_state=42)),
])

param_dist = {
    "clf__learning_rate": uniform(0.01, 0.3),
    "clf__max_depth": randint(3, 12),
    "clf__l2_regularization": uniform(0, 2),
}

search = RandomizedSearchCV(
    full_pipe, param_dist, n_iter=50,
    cv=StratifiedKFold(5, shuffle=True, random_state=42),
    scoring="roc_auc", n_jobs=-1, random_state=42
)
search.fit(X_train, y_train)

best_model = search.best_estimator_
print("Test AUC:", roc_auc_score(y_test, best_model.predict_proba(X_test)[:, 1]))
```
Wrapping preprocessing + model in one `Pipeline` guarantees no leakage during cross-validation, and lets you tune preprocessing hyperparameters (e.g., imputer strategy) alongside model hyperparameters.

---

## 12. Neural Network Basics (bridging into deep learning)

- **Perceptron**: single linear unit + step function — can only learn linearly separable functions.
- **MLP (Multi-Layer Perceptron)**: stacked layers of `linear transform → non-linear activation`. Universal function approximator with enough width/depth.
- **Activations**: ReLU (default, avoids vanishing gradients, cheap), sigmoid/tanh (saturate, used mainly at output for probabilities), softmax (multiclass output layer).
- **Backpropagation**: computes gradients of the loss w.r.t. every weight via the chain rule, layer by layer, backward from the output — this is what the optimizers in §4 actually operate on.
- **Vanishing/exploding gradients**: deep networks with saturating activations or poor initialization can have gradients shrink/blow up across layers — mitigated by ReLU, batch norm, residual connections, careful initialization (Xavier/He).

`sklearn`'s `MLPClassifier`/`MLPRegressor` are fine for small feed-forward nets; for CNNs (images), RNNs/Transformers (sequences/text), or large-scale training, use **PyTorch** or **TensorFlow/Keras**.

---

## 13. Algorithm Selection Cheat Sheet

| Situation | First choice |
|---|---|
| Tabular data, need strong baseline fast | Random Forest / HistGradientBoosting |
| Tabular data, need max accuracy, can tune | XGBoost / LightGBM / CatBoost |
| Need interpretability (coefficients, odds ratios) | Logistic / Linear Regression (+ L1/L2) |
| Many irrelevant features, want feature selection | Lasso, or tree-based feature importance |
| Text classification | Naive Bayes (baseline) → Linear SVM / Logistic Regression on TF-IDF → Transformers for SOTA |
| High-dimensional, small n | Linear SVM, Naive Bayes, Ridge/Lasso |
| Non-linear boundary, small-medium data | Kernel SVM (RBF), Gradient Boosting, Random Forest |
| Very large n (millions of rows) | SGDClassifier/Regressor, linear models, HistGradientBoosting, neural nets |
| Need probability estimates that are well-calibrated | Logistic Regression, or calibrate others with `CalibratedClassifierCV` |
| Unlabeled data, find natural groups | k-Means / DBSCAN / GMM |
| Reduce dimensions for modeling | PCA |
| Reduce dimensions for visualization only | t-SNE / UMAP |
| Images | CNNs (PyTorch/TF), not sklearn |
| Sequences/text (deep) | RNN/LSTM or Transformers |
| Time series forecasting | ARIMA/ETS (statsmodels), or gradient boosting with lag features, or Prophet |
| Anomaly/outlier detection | IsolationForest, One-Class SVM, DBSCAN noise points, Elliptic Envelope |

```python
from sklearn.ensemble import IsolationForest
iso = IsolationForest(contamination=0.02, random_state=42).fit(X_train)
outliers = iso.predict(X_test)  # -1 = anomaly, 1 = normal
```

---

## 14. Practical Checklist for a Real Project

1. Understand the business metric — pick the ML metric that actually correlates with it (precision vs recall tradeoff, cost-sensitive thresholds).
2. Split data correctly first (respect groups/time), before touching preprocessing.
3. Build a dumb baseline (majority class / mean predictor / simple linear model) — anything you ship must beat it.
4. Wrap preprocessing + model in a `Pipeline` to prevent leakage.
5. Cross-validate, don't trust a single split.
6. Start simple (linear/tree baseline), add complexity only if it earns its keep on validation data.
7. Check learning curves to diagnose bias vs variance before blindly tuning hyperparameters.
8. Tune with `RandomizedSearchCV`/Optuna, not manual grid guessing.
9. Inspect errors, not just aggregate metrics — look at confusion matrix, worst residuals, feature importances/SHAP values.
10. Validate on a held-out test set exactly once, and monitor for drift after deployment.
