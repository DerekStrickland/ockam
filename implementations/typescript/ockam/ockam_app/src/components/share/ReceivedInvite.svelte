<script>
import { invoke } from '@tauri-apps/api/tauri'
import ActionButton from './ActionButton.svelte'

export let invite;

async function accept(id) {
  await invoke('plugin:sharing|accept_invite', {id: id}).then((res) => console.log(res))
}
</script>

<tr>
  <th scope="row">{invite.id}</th>
  <td>{invite.recipient_email || ""}</td>
  <td>{invite.expires_at}</td>
  <td/>
  <td>{invite.scope}</td>
  <td>{invite.target_id}</td>
  <td>{invite.grant_role}</td>
  <td>
    <button disabled={invite.remaining_uses == 0} on:click={() => accept(invite.id)}>Accept</button>
  </td>
</tr>

