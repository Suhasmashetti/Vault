# 🏦 Vault Program

A Solana program built with [Anchor](https://book.anchor-lang.com/) that allows users to **deposit** and **withdraw** SPL tokens into a secure on-chain vault. This serves as the foundation for building lending platforms, staking protocols, or custodial services.

---

## 🚀 Introduction
The Vault program provides a simple and secure way to lock tokens into a program-owned account.  
It ensures that only authorized instructions can move funds in and out of the vault.  

This program is designed for:
- Token custody
- Building DeFi primitives
- Experimenting with Anchor-based smart contracts

---

## ✨ Features
- Deposit SPL tokens into a program-owned vault.  
- Withdraw SPL tokens securely.  
- PDA (Program Derived Address)-based authority for the vault.  
- Anchor framework integration for easier development.  

---

## ⚙️ How it Works
1. **Initialize Vault**  
   - A PDA vault account is created for storing tokens.  
2. **Deposit Tokens**  
   - Users send SPL tokens to the vault via CPI to the Token Program.  
3. **Withdraw Tokens**  
   - Users can withdraw tokens back from the vault (subject to rules/authority checks).  

---

## 📜 Program Details
- **Language:** Rust (via Anchor framework)  
- **Token Standard:** SPL Token  
- **Accounts:**
  - `VaultDataAccount` → Stores vault metadata  
  - `VaultTokenAccount` → PDA-owned token account  
  - `UserTokenAccount` → Token account of depositor/withdrawer  

---

## 🛠 Usage
### Build & Deploy
```bash
anchor build
anchor deploy


---
Example

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Vault as Program<Vault>;

async function main() {
  const amount = new anchor.BN(1000);

  // Example: Deposit tokens
  await program.methods
    .depositTokens(amount)
    .accounts({
      user: provider.wallet.publicKey,
      userTokenAccount: userTokenAccount,
      vaultTokenAccount: vaultTokenAccount,
      vaultData: vaultData,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    })
    .rpc();

  console.log("✅ Tokens deposited!");
}

main().catch(err => console.error(err));

---
Development

git clone https://github.com/your-repo/vault.git
cd vault
anchor build
