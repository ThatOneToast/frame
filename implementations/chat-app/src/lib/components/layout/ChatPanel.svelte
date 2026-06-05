<script lang="ts">
  import { ui } from '$lib/generated/generated';
  import MessageList from '$lib/components/messages/MessageList.svelte';
</script>

<main class={ui.ChatPanel} aria-label="Messages">
  <MessageList />
</main>
