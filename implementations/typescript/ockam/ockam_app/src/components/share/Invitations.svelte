<script>
import { invoke } from '@tauri-apps/api/tauri'
import InvitationTable from './InvitationTable.svelte'

let list_invites = invoke('plugin:sharing|list_invites')

async function refresh() {
  await invoke('plugin:sharing|refresh_invites')
  list_invites = invoke('plugin:sharing|list_invites')
}
</script>

<button on:click={refresh}>Refresh</button>

{#await list_invites}
  Loading...
{:then invites}
  <div>
    <InvitationTable invites={invites.sent} />
    <InvitationTable invites={invites.received} direction="received"/>
  </div>
{:catch error}
  {@debug error}
  Error!
{/await}
