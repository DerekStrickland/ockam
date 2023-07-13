<script>
export let disabled = false;
// TODO: receive callback function as prop
</script>

<button disabled={disabled}>
  <slot />
</button>
