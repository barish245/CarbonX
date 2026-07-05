# 🌍 CarbonX

<div align="center">

**Decentralized Carbon Credit Marketplace for SMEs**

*On-chain carbon credit verification, trading, and retirement secured by Stellar Soroban smart contracts*

[![Live Demo](https://img.shields.io/badge/Live_Demo-carbonx--stellar.netlify.app-16a34a?style=for-the-badge&logo=netlify)](https://carbonx-stellar.netlify.app/)
[![Vercel Demo](https://img.shields.io/badge/Vercel_Mirror-carbon--x--jade.vercel.app-000000?style=for-the-badge&logo=vercel)](https://carbon-x-jade.vercel.app/)
[![GitHub](https://img.shields.io/badge/Source_Code-barish245%2FCarbonX-181717?style=for-the-badge&logo=github)](https://github.com/barish245/CarbonX)
[![Network](https://img.shields.io/badge/Network-Stellar_Testnet-00B4D8?style=for-the-badge&logo=stellar)](https://stellar.expert/explorer/testnet)
[![Built for RiseIn](https://img.shields.io/badge/Built_for-RiseIn_Level_4-f59e0b?style=for-the-badge)](https://www.risein.com/)

</div>

---

## 📋 Table of Contents

1. [Problem Statement](#-problem-statement)
2. [Why Stellar?](#-why-stellar)
3. [Live Deployment](#-live-deployment)
4. [Contract Addresses & Transactions](#-contract-addresses--transactions)
5. [User Onboarding & Feedback](#-user-onboarding--feedback)
6. [Architecture](#-architecture)
7. [Smart Contracts](#-smart-contracts)
8. [Production Hardening (Level 4)](#-production-hardening-level-4)
9. [Tech Stack](#-tech-stack)
10. [Project Structure](#-project-structure)
11. [Testing](#-testing)
12. [CI/CD Pipeline](#-cicd-pipeline)
13. [Local Development](#-local-development)
14. [Roadmap](#-roadmap)
15. [Author](#-author)

---

## 🟢 Problem Statement

Small and Medium Enterprises (SMEs) represent 90% of global businesses, but they are effectively shut out of the voluntary carbon market.

| Issue | Impact |
|-------|--------|
| **Opaque Verification** | Carbon verification is prohibitively expensive (often $20k–$100k) and favors legacy brokers |
| **Double-Counting Risk** | Opacity in registries leads to double-counting and fraud, reducing corporate trust in offsets |
| **Settlement Delays** | Legacy trading settlements can take weeks and carry high international transaction fees |
| **Opaque Disputes** | Buyers lack public proof of retirement when offsetting credits for ESG compliance |

**CarbonX** replaces centralized brokers and registries with programmable, auditable Soroban smart contracts. SME developers submit carbon offsets, authorized auditors verify them on-chain, and buyers can purchase, trade, or permanently retire credits transparently with atomic settlements — removing intermediaries and enabling access for all.

---

## 🌟 Why Stellar?

CarbonX is not a generic blockchain application. It is a protocol that specifically requires Stellar's unique network architecture:

| Stellar Property | CarbonX Benefit |
|-----------------|-----------------|
| **~5 second finality** | Credit sales and payments settle instantly without settlement risk |
| **Sub-cent fees ($0.00001)** | Enables micro-tokenization of small-scale local projects (e.g. municipal solar/small farms) |
| **Soroban Inter-Contract Calls** | Coordinates Verification, Registry, Marketplace, and Settlement contracts atomically |
| **Built-in Assets / Trustlines** | Exposes secure, native tokenization for carbon offsets |
| **Fee Bump Transactions** | Gasless onboarding - sponsors transaction fees for non-crypto-native SMEs |

---

## 🌐 Live Deployment

| Resource | Link |
|----------|------|
| 🌍 **Live dApp (Netlify)** | [carbonx-stellar.netlify.app](https://carbonx-stellar.netlify.app/) |
| 🌍 **Live dApp (Vercel)** | [carbon-x-jade.vercel.app](https://carbon-x-jade.vercel.app/) |
| 🎬 **Demo Video** | [Google Drive — Walkthrough Recording](https://drive.google.com/file/d/1tI_jgWIF61P4U1DNgw2Ns23d2vVSuNb3/view?usp=sharing) |
| 💻 **GitHub Repo** | [barish245/CarbonX](https://github.com/barish245/CarbonX) |
| 📋 **User Feedback Form** | [CarbonX Feedback — Google Forms](https://forms.gle/4PRxsnXBNGUrtvrB9) |
| 📊 **Onboarded Users & Wallet Responses** | [Responses Spreadsheet — Google Sheets](https://docs.google.com/spreadsheets/d/1KexebLah9jnwBw1FSgdf25AH0dZRlAAfd98PtGuVFIE/edit?resourcekey=&gid=2100511142#gid=2100511142) |

---

## 🔗 Contract Addresses & Transactions

All contracts are deployed and cross-initialized on the **Stellar Testnet**.

### Deployed Contract IDs

| Contract | Address |
|----------|---------|
| **Verification Contract** | `CC7Q43DUGVPSKSXPOYT4TTSXTXEJAICL7N6LT5GB4ZN5UR3Y6T5VKVFQ` |
| **Carbon Registry** | `CAV5ID4RDPAC5DOMOKRWH2YUORQHYRGJBWRTXG33RLBCKDMS333FA2MD` |
| **Marketplace** | `CC4SBE33FC2AL3K77BQIJOFHO7RVDAKQFVTOLQARF6SHLOTIYV2MXYIY` |
| **Settlement** | `CDXRNXG33D2MGTWX7VZJ3WO5ORHPOUTQN2WGQP6AJNJQUBD7TSUWXN5P` |
| **Retirement** | `CCKX33TYO6FRCJV4BDOO73VVCKIOTF6DNZH6KPAWOXVBML7UGTK7V5JH` |

- **Transaction Hash (Verification Init)**: `0x1f0d36675fd2884a229a8f273be8a74e50d60c49`

---

## 👥 User Onboarding & Feedback

As part of the Level 4 production MVP requirements, we onboarded real users to validate the complete carbon credit lifecycle on the Stellar Testnet.

**Onboarding Journey:**

```
1. SME registers on CarbonX → Funds testnet account via Friendbot
2. Developer submits carbon offset project
3. Auditor verifies project (Triggering Registry minting via Inter-Contract Call)
4. Developer lists credits on the Marketplace
5. Buyer purchases listing (XLM settles to seller; credit ownership updates)
6. Buyer retires credits (Generating Retirement Certificate & updating Carbon Score)
7. User submits feedback via the Google Form
```

| Resource | Link |
|----------|------|
| 📋 **Feedback Form** | [Submit Feedback](https://forms.gle/4PRxsnXBNGUrtvrB9) |
| 📊 **User Responses & Wallet Proof** | [View Spreadsheet](https://docs.google.com/spreadsheets/d/1KexebLah9jnwBw1FSgdf25AH0dZRlAAfd98PtGuVFIE/edit?resourcekey=&gid=2100511142#gid=2100511142) |

---

## 🏗️ Architecture

CarbonX is composed of 5 Soroban smart contracts that communicate via Inter-Contract Calls (ICC), and a Next.js frontend that builds and submits signed Stellar transactions.

```
                  ┌─────────────────┐
                  │ Verification SC │
                  └────────┬────────┘
                           │
                     (ICC: mint)
                           ▼
                  ┌─────────────────┐
                  │ Carbon Registry │◀────────────────┐
                  └────────┬────────┘                 │
                           │                          │
                 (ICC: transfer)               (ICC: retire)
                           ▼                          │
                  ┌─────────────────┐        ┌────────┴────────┐
                  │ Marketplace SC  │        │  Retirement SC  │
                  └────────┬────────┘        └─────────────────┘
                           │
                     (ICC: settle)
                           ▼
                  ┌─────────────────┐
                  │  Settlement SC  │
                  └─────────────────┘
```

### Inter-Contract Communication (ICC) Flow

The ICC design is the architectural centerpiece of CarbonX. All registry state changes are triggered atomically:
1. **Verification ➡️ Registry**: When the verifier approves a project, the Verification Contract uses ICC to mint credits to the developer.
2. **Marketplace ➡️ Settlement & Registry**: When a buyer purchases a listing, the Marketplace contract locks/settles XLM via the Settlement Contract and transfers the registry token ownership via the Registry contract.
3. **Retirement ➡️ Registry**: When a user retires credits, the Retirement Contract marks them retired in the Registry and issues a certificate.

---

## 📜 Smart Contracts

### 1. 🔍 Verification Contract (`verification-contract`)
- Registers authorized verifiers.
- Submits carbon reduction projects.
- Calls `CarbonRegistry` to mint credits upon verification.

### 2. 📋 Carbon Registry (`carbon-registry`)
- Tracks total supply, individual credit ownership balances, and retirement states.
- Restricts minting to the Verification Contract only.

### 3. 🏪 Marketplace Contract (`marketplace-contract`)
- Holds listed credits in contract-managed escrow.
- Facilitates buy and listing cancel transactions.

### 4. 💸 Settlement Contract (`settlement-contract`)
- Emits lock and release payment events.
- Handles payments between buyers and sellers.

### 5. 🏆 Retirement Contract (`retirement-contract`)
- Permanently retires credits via the Registry.
- Updates the buyer's Carbon Score and issues certificates.

---

## 🛡️ Production Hardening (Level 4)

The following security audits and production improvements were implemented and tested in Level 4:

### Smart Contract Hardening
- **Initialization Guard**: Prevents contracts from being reinitialized after deployment.
- **Milestone & Input Bounds Check**: Enforces positive offset volumes and non-empty project names.
- **Double-spending Escrow Guard**: Enforces that listed credits are locked in the marketplace contract, preventing double-selling.
- **Instance TTL Extension**: Extends the storage life of ledger entries to prevent expiry.

### Frontend Production Quality
- **Telemetry System**: Created a visual telemetry interface logging user actions and Stellar transaction lifetimes in real time.
- **Transactional Spinner states**: Added clear, non-blocking loading states during wallet interaction and Friendbot funding.
- **Graceful Error Banners**: Intercepts wallet cancellations and underfunded account errors with helpful user guidance.
- **Floating Feedback Button**: Easily accessible feedback loop in the main layout pointing to the Google Form.

### Monitoring & Analytics
- **Vercel Analytics & Speed Insights**: Integrated tracking in `layout.js` to monitor page loads, speed indexes, and client errors.
- **Google Form responses database**: Connected to Excel/Sheets logs to verify 10+ wallet interactions.

---

## 📸 Submission Screenshots

### 🖥️ Carbon Marketplace UI
![Marketplace](sub%20assets/ui2.png)

### 📱 Diagnostics & Telemetry Dashboard
![Diagnostics](sub%20assets/ui1.png)

### 📈 Vercel Web Analytics
![Vercel Analytics](sub%20assets/analytics.png)

### 🔄 CI/CD Pipeline
![CI/CD Pipeline](sub%20assets/cicd.png)

---

## 🧪 Testing

### Test Summary

| Suite | Tests | Status |
|-------|-------|--------|
| Frontend (Vitest) | 3 tests | ✅ All Passing |
| Rust Contracts | 1 test (Full Flow) | ✅ All Passing |

```bash
# Run contract tests
cd contracts && cargo test

# Run frontend tests
cd frontend && npm test
```

---

## 🛠️ Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Frontend Framework** | Next.js 16 (App Router) | SSR, client routing, production builds |
| **blockchain SDK** | @stellar/stellar-sdk | Transaction building & RPC interaction |
| **Wallet Integration** | StellarWalletsKit & Freighter | Freighter login and testnet signatures |
| **Smart Contracts** | Soroban (Rust) | On-chain registry, verification & trade logic |
| **Analytics & Monitoring** | Vercel Analytics | Real-time web performance & crash reports |
| **CI/CD** | GitHub Actions | Automated lint, build, and test pipeline |

---

## 📁 Project Structure

```
CarbonX/
├── .github/
│   └── workflows/
│       └── main.yml              # CI/CD Pipeline (GitHub Actions)
├── contracts/                    # Soroban Smart Contracts (Rust)
│   ├── carbon-registry/          # Core credit ledger
│   ├── marketplace-contract/     # Escrow & listing logic
│   ├── retirement-contract/      # Credit retirement
│   ├── settlement-contract/      # Payment logic
│   └── verification-contract/    # Auditor & submission logic
├── frontend/                     # Next.js App
│   ├── __tests__/                # Vitest unit tests
│   ├── src/
│   │   ├── app/
│   │   │   ├── page.jsx          # Main application file
│   │   │   └── layout.js         # Layout with Vercel Analytics
│   │   └── lib/
│   │       ├── stellar.js        # Stellar helpers
│   │       └── telemetry.js      # Telemetry logging helper
```

---

## 🗺️ Roadmap

### ✅ Level 3 (Complete)
- 5 Soroban smart contracts with Inter-Contract Communication.
- Next.js frontend with Freighter wallet integration.
- Real-time contract event logging dashboard.

### ✅ Level 4 (Complete)
- Visual Telemetry & Diagnostics dashboard.
- Clear transactional loading spinners and error alerts.
- Vercel Analytics and Speed Insights tracking.
- Floating Google Form feedback button.
- 10+ real users onboarded with wallet proofs saved in Sheets.
- Clean Git log under new author `barish245`.

### 🔜 Level 5 (Planned)
- On-chain verifier reputation system.
- Advanced carbon project search and categorization filters.
- Enterprise pitch deck for voluntary carbon credits markets.
- Scale from 10 to 50 active users.

### 🔜 Level 6 (Planned)
- Security audit of all 5 contracts.
- Mainnet deployment.
- Fee Sponsorship for gasless SME onboarding.

---

## 👨💻 Author

**Praveen Garakot (barish245)** — [@barish245](https://github.com/barish245)

*Built for the RiseIn Stellar Bootcamp — Level 4 Black Belt*
