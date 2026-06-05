<script lang="ts">
  import { ui } from '$lib/generated/generated';
  import ChannelList from '$lib/components/channels/ChannelList.svelte';
  import { activeServer } from '$lib/stores/chat';
</script>

<section class={ui.ChannelPanel} aria-label="Channels">
  <div class={ui.PanelHeader}>
    <span class={ui.PanelTitle}>{$activeServer?.name ?? 'Workspace'}</span>
    <span class={ui.PanelMeta}>online</span>
  </div>

  <ChannelList />
</section>
