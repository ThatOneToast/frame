<script lang="ts">
  import { ui } from '$lib/generated/generated';
  import ChannelRow from './ChannelRow.svelte';
  import { channels } from '$lib/stores/chat';
</script>

<nav class={ui.ChannelList} aria-label="Channel list">
  <div class={ui.SectionLabel}>text</div>
  {#each $channels.filter((channel) => channel.kind !== 'voice') as channel}
    <ChannelRow {channel} />
  {/each}

  <div class={ui.SectionLabel}>voice</div>
  {#each $channels.filter((channel) => channel.kind === 'voice') as channel}
    <ChannelRow {channel} />
  {/each}
</nav>
