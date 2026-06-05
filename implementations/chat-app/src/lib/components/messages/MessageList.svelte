<script lang="ts">
  import { ui } from '$lib/generated/generated';
  import MessageItem from './MessageItem.svelte';
  import { messages, users } from '$lib/stores/chat';
</script>

<div class={ui.MessageList} role="list" aria-label="Channel messages">
  {#each $messages as message}
    <MessageItem {message} author={$users.find((user) => user.id === message.authorId)} />
  {/each}
</div>
