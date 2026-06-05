<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/layout/AppShell.svelte';
  import { initializeChatApp } from '$lib/stores/chat';

  onMount(() => {
    void initializeChatApp();
  });
</script>

<AppShell />
