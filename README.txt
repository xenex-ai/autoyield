AutoYield – Modularer DeFi Yield Aggregator auf Solana

Beschreibung:
AutoYield ist ein programmierbarer, on-chain DeFi-Yield-Aggregator auf Solana, der:
1. Zero-Copy-Strategien kapselt (z.B. Raydium, Serum, Lending).
2. Paralleles Harvesting über Sealevel CPI-Aufrufe.
3. Permissionless Plugins und Governance-Unterstützung bietet.

Setup:
1. Installiere Rust und Anchor:
   https://book.anchor-lang.com/chapter_2/installation.html
2. Clone das Repo und wechsle ins Projektverzeichnis:
   git clone <repo-url>
   cd autoyield
3. Baue das Programm:
   anchor build
4. Starte das Solana Devnet:
   solana-test-validator
5. Initialisiere die lokale Registry:
   anchor deploy
6. Wechsle ins Client-Verzeichnis, installiere Abhängigkeiten und führe das Script aus:
   cd client
   npm install
   npm run start

Credits:
- ChatGPT (dich)
- xenexAi
