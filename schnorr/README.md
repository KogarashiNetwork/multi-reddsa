# Schnorr Signature
Implementation of [Efficient Signature Generation by Smart Cards](https://link.springer.com/article/10.1007/BF00196725)

## Params

- p: order of scalar field
- g: basepoint of prime order elliptic curve group
- H: hash function $H: \{0,1\}^* \rightarrow \mathbb Z_q$
- m: message to be signed

## Key Generation

- private key: $x \in \mathbb Z_q$
- public key: $y = x * g$

## Sign

- choose random $k \in \mathbb Z_q$
- let $r = k * g$
- let $e = H(r || M)$
- let $s = k - xe$
- let $(s, e)$ signature

## Verify

- let $r_v = s * g + e * y$
- let $e_v = H(r_v || M)$

if $e_v = e$, the signature is valid.
