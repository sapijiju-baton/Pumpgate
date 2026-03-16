# PumpGate

**Repository:** https://github.com/sapijiju-baton/Pumpgate  
**Author:** https://github.com/sapijiju-baton  
**Co-author:** https://github.com/a1lon9-baton  
**Contributors:**  
- https://github.com/andrei-baton  
- https://github.com/arv-baton  
- https://github.com/drew-baton  

**Organization:** https://batoncorporation.com

---

## What is PumpGate

PumpGate is an on-chain token gate wrapper for pump.fun launches built on Solana.

PumpGate requires a wallet to hold a minimum of 25,000 $PUMP tokens before it will forward a token launch instruction to pump.fun. If the wallet does not hold enough $PUMP, the program throws a custom error and the launch is rejected entirely.

This demonstrates a simple, verifiable, on-chain mechanism by which pump.fun could require $PUMP token holdings as a prerequisite for launching on the platform — creating direct utility for $PUMP and a meaningful barrier against low-quality or spam launches.

---

## For pump.fun Users

Under a system like PumpGate, you would need to hold 25,000 $PUMP in your wallet before being able to launch a token on pump.fun. This is enforced at the program level — not a UI check, not bypassable.

---

## On-chain Verification

**Mainnet Program ID:** `REPLACE_WITH_PROGRAM_ID_AFTER_DEPLOY`

Verify on Solana Explorer:  
https://explorer.solana.com/address/REPLACE_WITH_PROGRAM_ID_AFTER_DEPLOY

---

*This is a proof of concept deployed on Solana mainnet. Built by Baton Corporation.*
