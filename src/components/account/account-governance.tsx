'use client'

import { PublicKey } from '@solana/web3.js'

export function AccountGovernance({ address }: { address: PublicKey }) {
  return (
    <div>
      <h2 className="text-2xl font-bold">Governance</h2>
      <p>Implement governance-related features here. Address: <code>{address.toString()}</code></p>
    </div>
  )
}