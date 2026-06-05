<script lang="ts">
  import { ui } from '$lib/generated/generated';
  import UserRow from './UserRow.svelte';
  import { users } from '$lib/stores/chat';
</script>

<div class={ui.UserList} role="list" aria-label="Member list">
  {#each $users as user}
    <UserRow {user} />
  {/each}
</div>
