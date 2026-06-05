<script lang="ts">
  import { ui } from '$lib/generated/generated';
  import { activeServerId, selectServer, servers } from '$lib/stores/chat';
</script>

<aside class={ui.SidebarRail} aria-label="Servers">
  <div class={ui.SidebarTop}>
    <span class={ui.RailLabel}>srv</span>
  </div>

  <nav class={ui.ServerList} aria-label="Server list">
    {#each $servers as server}
      <button
        class={`${ui.ServerButton} ${server.id === $activeServerId ? ui.ServerButtonActive : ''}`}
        type="button"
        aria-label={`Open ${server.name}`}
        aria-current={server.id === $activeServerId ? 'page' : undefined}
        onclick={() => void selectServer(server.id)}
      >
        <span>{server.abbreviation}</span>
        {#if server.unreadCount > 0}
          <span class={ui.UnreadDot} aria-label={`${server.unreadCount} unread`}></span>
        {/if}
      </button>
    {/each}
  </nav>
</aside>
