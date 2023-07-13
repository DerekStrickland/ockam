<script>
import ActionButton from './ActionButton.svelte'

export let invite;
</script>

<tr>
  <th scope="row">{invite.id}</th>
  <td>{invite.recipient_email || ""}</td>
  <td>{invite.expires_at}</td>
  <td>{invite.remaining_uses}</td>
  <td>{invite.scope}</td>
  <td>{invite.target_id}</td>
  <td>{invite.grant_role}</td>
  <td>
    {#if invite.status == "Online"}
      <ActionButton disabled>Services</ActionButton>
      <ActionButton disabled>Disconnect</ActionButton>
    {/if}
    {#if invite.remaining_uses > 0}
      <ActionButton disabled>Revoke</ActionButton>
    {/if}
  </td>
</tr>
