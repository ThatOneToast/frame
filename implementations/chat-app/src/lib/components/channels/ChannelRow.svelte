<script lang="ts">
  import { ui } from '$lib/generated/generated';
  import type { Channel } from '$lib/models/channel';
  import { activeChannelId, selectChannel } from '$lib/stores/chat';

  export let channel: Channel;
</script>

<button
  class={`${ui.ChannelButton} ${channel.id === $activeChannelId ? ui.ChannelButtonActive : ''}`}
  type="button"
  aria-current={channel.id === $activeChannelId ? 'page' : undefined}
  onclick={() => void selectChannel(channel.id)}
>
  <span class={ui.ChannelHash}>{channel.kind === 'voice' ? '>' : '#'}</span>
  <span class={ui.ChannelName}>{channel.name}</span>
  {#if channel.unreadCount > 0}
    <span class={ui.ChannelBadge}>{channel.unreadCount}</span>
  {/if}
</button>
