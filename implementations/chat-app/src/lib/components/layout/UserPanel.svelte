<script lang="ts">
  import { ui } from '$lib/generated/generated';
  import UserList from '$lib/components/users/UserList.svelte';
</script>

<aside class={ui.UserPanel} aria-label="Online users">
  <div class={ui.PanelHeader}>
    <span class={ui.PanelTitle}>members</span>
    <span class={ui.PanelMeta}>live</span>
  </div>

  <UserList />
</aside>
