<script>
import ReceivedInvite from './ReceivedInvite.svelte'
import SentInvitation from './SentInvitation.svelte'

export let direction = "sent";
export let invites;

$: comp = direction == "sent" ? SentInvitation : ReceivedInvite
$: title = direction == "sent" ? "Sent Invitations" : "Received Invitations"
$: email_header = direction == "sent" ? "Recipient" : "From"

</script>

<div>
  <h2>{title}</h2>
  <table>
    <thead>
      <tr>
        <th>Invite ID</th>
        <th>{email_header}</th>
        <th>Expires At</th>
        <th>Remaining Uses</th>
        <th>Scope</th>
        <th>Target ID</th>
        <th>Grants Role</th>
        <th>Actions</th>
      </tr>
    </thead>
    <tbody>
      {#each invites as invite}
        <svelte:component this={comp} {invite} />
      {/each}
    </tbody>
  </table>
</div>
