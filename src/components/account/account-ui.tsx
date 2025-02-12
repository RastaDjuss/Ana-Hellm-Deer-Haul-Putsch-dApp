'use client'

import { useWallet } from '@solana/wallet-adapter-react'
import { LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'
import { IconRefresh } from '@tabler/icons-react'
import { useQueryClient } from '@tanstack/react-query'
import { useMemo, useState, useCallback } from 'react'
import { AppModal, ellipsify } from '../ui/ui-layout'
import { useCluster } from '../cluster/cluster-data-access'
import { ExplorerLink } from '../cluster/cluster-ui'
import {
  useGetBalance,
  useGetSignatures,
  useGetTokenAccounts,
  useRequestAirdrop,
  useTransferSol,
} from './account-data-access'

// Vérification d'une clé publique valide
function isValidPublicKey(destination: string): boolean {
  try {
    new PublicKey(destination)
    return true
  } catch (err) {
    return false
  }
}

// Composant pour afficher le solde en SOL
function BalanceSol({ balance }: { balance: number }) {
  return <span>{Math.round((balance / LAMPORTS_PER_SOL) * 100000) / 100000} SOL</span>
}

// Vérification du compte et affichage de la balance
export function AccountBalance({ address }: { address: PublicKey }) {
  const query = useGetBalance({ address })

  return (
    <div>
      <h1 className="text-5xl font-bold cursor-pointer" onClick={() => query.refetch()}>
        {query.data ? <BalanceSol balance={query.data} /> : '...'}
      </h1>
    </div>
  )
}

// Vérification si un portefeuille est connecté
export function AccountChecker() {
  const { publicKey } = useWallet()
  if (!publicKey) {
    return null
  }
  return <AccountBalanceCheck address={publicKey} />
}

export function AccountBalanceCheck({ address }: { address: PublicKey }) {
  const { cluster } = useCluster()
  const mutation = useRequestAirdrop({ address })
  const query = useGetBalance({ address })

  if (query.isLoading) {
    return null
  }
  if (query.isError || !query.data) {
    return (
      <div className="alert alert-warning text-warning-content/80 rounded-none flex justify-center">
        <span>
          Vous êtes connecté au cluster <strong>{cluster.name}</strong>, mais votre compte est introuvable.
        </span>
        <button
          className="btn btn-xs btn-neutral"
          onClick={() =>
            mutation
              .mutateAsync(1)
              .catch((err) => console.error('Airdrop error:', err))
          }
        >
          Demander un airdrop
        </button>
      </div>
    )
  }
  return null
}

// Boutons d'action sur les comptes – envoyer, recevoir, demander airdrop
export function AccountButtons({ address }: { address: PublicKey }) {
  const wallet = useWallet()
  const { cluster } = useCluster()
  const [showAirdropModal, setShowAirdropModal] = useState(false)
  const [showReceiveModal, setShowReceiveModal] = useState(false)
  const [showSendModal, setShowSendModal] = useState(false)

  const toggleAirdropModal = useCallback(() => setShowAirdropModal((prev) => !prev), [])
  const toggleReceiveModal = useCallback(() => setShowReceiveModal((prev) => !prev), [])
  const toggleSendModal = useCallback(() => setShowSendModal((prev) => !prev), [])

  return (
    <div>
      <ModalAirdrop hide={toggleAirdropModal} address={address} show={showAirdropModal} />
      <ModalReceive hide={toggleReceiveModal} address={address} show={showReceiveModal} />
      <ModalSend hide={toggleSendModal} address={address} show={showSendModal} />

      <div className="space-x-2">
        <button
          disabled={cluster.network?.includes('mainnet')}
          className="btn btn-xs lg:btn-md btn-outline"
          onClick={toggleAirdropModal}
        >
          Airdrop
        </button>
        <button
          disabled={wallet.publicKey?.toString() !== address.toString()}
          className="btn btn-xs lg:btn-md btn-outline"
          onClick={toggleSendModal}
        >
          Envoyer
        </button>
        <button className="btn btn-xs lg:btn-md btn-outline" onClick={toggleReceiveModal}>
          Recevoir
        </button>
      </div>
    </div>
  )
}

// Visualisation des tokens liés à un compte
export function AccountTokens({ address }: { address: PublicKey }) {
  const [showAll, setShowAll] = useState(false)
  const query = useGetTokenAccounts({ address })
  const client = useQueryClient()

  const items = useMemo(() => {
    if (showAll) return query.data
    return query.data?.slice(0, 5)
  }, [query.data, showAll])

  return (
    <div className="space-y-2">
      <div className="flex justify-between">
        <h2 className="text-2xl font-bold">Comptes de Tokens</h2>
        {query.isLoading ? (
          <span className="loading loading-spinner"></span>
        ) : (
          <button
            className="btn btn-sm btn-outline"
            onClick={async () => {
              await query.refetch()
              await client.invalidateQueries({
                queryKey: ['getTokenAccountBalance'],
              })
            }}
          >
            <IconRefresh size={16} />
          </button>
        )}
      </div>
      {query.isError && <pre className="alert alert-error">Erreur: {query.error?.message}</pre>}

      {query.data?.length === 0 ? (
        <div>Pas de comptes de tokens trouvés.</div>
      ) : (
        <table className="table border-4 rounded-lg border-separate border-base-300">
          <thead>
          <tr>
            <th>Clé Publique</th>
            <th>Mint</th>
            <th className="text-right">Solde</th>
          </tr>
          </thead>
          <tbody>
          {items?.map(({ account, pubkey }) => (
            <tr key={pubkey.toString()}>
              <td>
                <ExplorerLink label={ellipsify(pubkey.toString())} path={`account/${pubkey.toString()}`} />
              </td>
              <td>
                <ExplorerLink
                  label={ellipsify(account.data.parsed.info.mint)}
                  path={`account/${account.data.parsed.info.mint}`}
                />
              </td>
              <td className="text-right">{account.data.parsed.info.tokenAmount.uiAmount}</td>
            </tr>
          ))}
          </tbody>
        </table>
      )}
    </div>
  )
}

// Historique des transactions pour un compte
export function AccountTransactions({ address }: { address: PublicKey }) {
  const query = useGetSignatures({ address })
  const [showAll, setShowAll] = useState(false)

  const items = useMemo(() => {
    if (showAll) return query.data
    return query.data?.slice(0, 5)
  }, [query.data, showAll])

  return (
    <div>
      <h2 className="text-2xl font-bold">Historique des Transactions</h2>
      {query.isError && <pre className="alert alert-error">Erreur: {query.error?.message}</pre>}
      {items?.map((item) => (
        <div key={item.signature}>
          <ExplorerLink label={ellipsify(item.signature)} path={`tx/${item.signature}`} />
        </div>
      ))}
    </div>
  )
}

// Modale pour AirDrop
function ModalAirdrop({ hide, show, address }: { hide: () => void; show: boolean; address: PublicKey }) {
  const mutation = useRequestAirdrop({ address })
  const [amount, setAmount] = useState('2')

  return (
    <AppModal
      hide={hide}
      show={show}
      title="Demande Airdrop"
      submitDisabled={!amount || mutation.isPending}
      submit={() =>
        mutation
          .mutateAsync(parseFloat(amount))
          .then(() => hide())
          .catch((err) => console.error('Airdrop Error:', err))
      }
    >
      <input
        type="number"
        min="1"
        value={amount}
        onChange={(e) => setAmount(e.target.value)}
        className="input input-bordered w-full"
      />
    </AppModal>
  )
}

// Modale pour Envoyer des fonds
function ModalSend({ hide, show, address }: { hide: () => void; show: boolean; address: PublicKey }) {
  const wallet = useWallet()
  const mutation = useTransferSol({ address })
  const [destination, setDestination] = useState('')
  const [amount, setAmount] = useState('1')

  return (
    <AppModal
      hide={hide}
      show={show}
      title="Envoyer des SOL"
      submitDisabled={!destination || !amount || mutation.isPending}
      submit={() => {
        if (!isValidPublicKey(destination)) {
          alert("Invalid Public Key")
          return
        }
        mutation.mutateAsync({
          destination: new PublicKey(destination),
          amount: parseFloat(amount),
        })
      }}
    >
      <input type="text" value={destination} onChange={(e) => setDestination(e.target.value)} />
      <input type="number" value={amount} onChange={(e) => setAmount(e.target.value)} />
    </AppModal>
  )
}