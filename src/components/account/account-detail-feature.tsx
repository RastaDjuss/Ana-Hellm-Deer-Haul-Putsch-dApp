'use client'

import { PublicKey } from '@solana/web3.js'
import { useMemo } from 'react'
import { useParams } from 'next/navigation'

import { AppHero, ellipsify } from '../ui/ui-layout'
import { ExplorerLink } from '../cluster/cluster-ui'
import { AccountBalance, AccountButtons, AccountTokens, AccountTransactions } from './account-ui'
import { AccountGovernance } from './account-governance' // Si ce composant est manquant, créez un boilerplate.

// Fonction parentale qui gère tous les détails de l'adresse publique
export default function AccountDetailFeature() {
  const params = useParams() // Obtenir les paramètres d'URL
  const address = useMemo(() => {
    if (!params.address) {
      return null
    }
    try {
      return new PublicKey(params.address)
    } catch (e) {
      console.error(`Invalid public key`, e)
      return null
    }
  }, [params])

  if (!address) {
    return <div className="alert alert-error">Error: Invalid account address.</div>
  }

  return (
    <div>
      <AppHero
        title={<AccountBalance address={address} />}
        subtitle={
          <div className="my-4">
            <ExplorerLink label={ellipsify(address.toString())} path={`account/${address}`} />
          </div>
        }
      >
        <div className="my-4">
          <AccountButtons address={address} />
        </div>
      </AppHero>

      {/* Insérez votre structure ici */}
      <div className="space-y-8">
        <AccountGovernance address={address} />
        <AccountTokens address={address} />
        <AccountTransactions address={address} />
      </div>
    </div>
  )
}