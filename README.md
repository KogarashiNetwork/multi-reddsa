# MuSig with RedDSA

## Jubjub Curve

**Equation**

$$
ax^2 + y^2 \equiv 1 + dx^2y^2
$$

**Addition Law**

$$
(x_3 = \frac{x_1y_1 + y_1x_1}{1 + dx_1x_1y_1y_1}, y_3 = \frac{y_1y_1 + ax_1x_1}{1 - dx_1x_1y_1y_1})
$$

## Schnorr Signature

**Key Generation**

- private key: $x \in \mathbb F_q$
- public key: $y = x * g$

**Sign**

- choose random $k \in \mathbb F_q$
- let $r = k * g$
- let $e = H(r || m)$
- let $s = k - xe$
- let $(s, e)$ signature

**Verify**

- let $r_v = s * g + e * y$
- let $e_v = H(r_v || M)$

if $e_v = e$, the signature is valid.

## MuSig

**KeyGen**

- private keys: $x_1,..,x_i \in F_q$
- public keys: $X_1,...,X_i = x_1 * g,..,x_i * g \in E(F_q)$

**PublicParams**

- aggregated public key: $\overline X = \prod_{i=1}^nX^{a_i}_i$
- randomness: chooses $r_i \in F_q$, computes $R_i = g^{r_i}$ and $t_i=H_{com}(R_i)$
- aggregated randomness: $R = \prod_{i=1}^nR_i$
- aggregated challenge: $c = H_{sig}(\overline X, R, m)$

**Sign**

- let $s_i = r_i + ca_ix_i$
- let $s = \sum_{i=1}^ns_i$
- let $(R, s)$ signature

**Verify**

- let $a_i = H_{agg}(L, X_i)$ for $i \in {1,2,...,i}$
- let $\overline X = \prod_{i=1}^nX^{a_i}_i$
- let $c = H_{sig}(\overline X, R, m)$
- let $r_v = s * g$
