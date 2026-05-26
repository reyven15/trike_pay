# TrikePay PH
**Instant, change-free fare payments for tricycle commuters in the Philippines.**

## Problem & Solution
**Problem:** A tricycle driver in Quezon City loses potential income because they cannot provide change for large bills, and passengers frequently lack small coins ("barya").
**Solution:** Commuters scan a QR code to pay via PHP-stablecoins. A Soroban contract splits the payment instantly between the driver and a tiny platform fee, ensuring the driver is paid regardless of physical change availability.

## Timeline
- **Build Phase:** 2 weeks (Contract + Frontend)
- **Beta Test:** 1 week (Local tricycle TODA in QC)
- **Demo Ready:** End of Week 4

## Stellar Features Used
- **Soroban Smart Contracts:** Automated fee splitting and earning logs.
- **PHP-Stablecoins:** Local currency stability for fares.
- **Atomic Operations:** Guaranteed payment to both driver and admin.

## Vision and Purpose
To digitize the informal transport sector in SEA, starting with the Philippines. By building an on-chain record of daily earnings, drivers can finally access micro-loans based on their transaction history rather than predatory lenders.

## Prerequisites
- Rust `v1.70+`
- Soroban CLI `v20.0.0+`
- Target `wasm32-unknown-unknown`

## How to Build
```bash
soroban contract build

## deployed contract
 https://stellar.expert/explorer/testnet/contract/CASZ5I3MA4C7F567WW6T4A3OEXCI2BFINPMUVKWEO35GPDW6FV5XCX44

 Contract ID: CASZ5I3MA4C7F567WW6T4A3OEXCI2BFINPMUVKWEO35GPDW6FV5XCX44

<img width="1365" height="767" alt="Screenshot 2026-05-26 182314" src="https://github.com/user-attachments/assets/fc33fe39-29ad-4cf3-9326-de35d8a034b2" />




