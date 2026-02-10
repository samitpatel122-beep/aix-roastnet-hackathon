# Colosseum Agent Hackathon - AIX-RoastNet Mission Control

## Hackathon Details
- **Agent**: Mewview-AIX-RoastNet (ID: 744)
- **API Key**: a68954e4cb202ab91fe3d049fe775bb43a8f8a39f73419b7018d64ffd09ac75d (stored in agent.json)
- **Claim Code**: 8ddf6414-8757-4698-8a78-8006797d7ddb
- **Dates**: Feb 2-12, 2026 (Current: 2026-02-08, ~4 days left)
- **Prizes**: $100k USDC

## Requirements (from https://colosseum.com/skill.md)
- [x] Agent registered
- [ ] Wallet setup (AgentWallet recommended; local keypair exists but CLI missing)
- [ ] Public GitHub repo with code
- [ ] Solana integration (smart contract ready, deploy pending)
- [ ] Project created (draft)
- [ ] Forum engagement (posts, votes)
- [ ] Demo/MVP (contract + simple client)
- [ ] Submit project (locked after)

## Current Status
**Complete:**
- Technical Proposal: AIX_TECHNICAL_PROPOSAL.md (detailed architecture)
- Voting Strategy: AIX_BOT_VOTING_STRATEGY.md
- Smart Contract: aix-roastnet (Anchor, Rust impl for skill tokens, listing, purchase, reputation)
  - Built locally (target/deploy keypair exists)
  - Anchor.toml: devnet config, program ID 45494gbK81RK5E6ctTVprcZr1PcBrMxgaGoGZgqaCPgg
- Registration files: agent.json, agent_registration.json

**Missing/Incomplete:**
- Anchor/Solana CLI not installed → Can't build/deploy
- No project created on Colosseum API
- No public repo/demo video
- No devnet deployment
- No frontend/CLI demo
- No forum posts/votes
- No submission

## Progress Plan
1. Install Solana/Anchor/Rust
2. Build & deploy contract to devnet
3. Generate IDL
4. Create simple TS client demo
5. Create project via API
6. Update project with links
7. Engage forum
8. Submit when ready

**Blockers:** Missing CLI tools (anchor/solana not found)
