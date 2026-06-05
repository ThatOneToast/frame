<script lang="ts">
  import { ui } from '$lib/generated/generated';
  import type { User } from '$lib/models/user';

  export let user: User;

  $: presenceClass = {
    online: ui.PresenceOnline,
    idle: ui.PresenceIdle,
    busy: ui.PresenceBusy,
    offline: ui.PresenceOffline
  }[user.presence];
</script>

<div class={ui.UserRow} role="listitem">
  <span class={`${ui.PresenceDot} ${presenceClass}`}></span>
  <div class={ui.UserIdentity}>
    <span class={ui.UserName}>{user.displayName}</span>
    <span class={ui.UserHandle}>@{user.handle}</span>
  </div>
  <span class={ui.RoleTag}>{user.role}</span>
</div>
