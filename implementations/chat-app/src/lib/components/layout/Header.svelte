<script lang="ts">
  import { ui } from '$lib/generated/generated';
  import { activeChannel, activeServer } from '$lib/stores/chat';
</script>

<header class={ui.HeaderBar}>
  <div class={ui.HeaderIdentity}>
    <span class={ui.AppMark}>FRAME</span>
    <span class={ui.HeaderTitle}>{$activeServer?.name ?? 'Loading workspace'}</span>
  </div>

  <div class={ui.HeaderChannel} aria-live="polite">
    <span class={ui.ChannelHash}>#</span>
    <span>{$activeChannel?.name ?? 'select-channel'}</span>
    <span class={ui.HeaderTopic}>{$activeChannel?.topic ?? 'Preparing channel state'}</span>
  </div>

  <nav class={ui.HeaderActions} aria-label="Workspace controls">
    <button class={ui.IconButton} type="button" aria-label="Open command palette">cmd</button>
    <button class={ui.IconButton} type="button" aria-label="Open settings">cfg</button>
  </nav>
</header>
