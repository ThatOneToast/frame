<script lang="ts">
  import { ui } from '$lib/generated/generated';
  import Sidebar from './Sidebar.svelte';
  import ChannelPanel from './ChannelPanel.svelte';
  import ChatPanel from './ChatPanel.svelte';
  import Header from './Header.svelte';
  import UserPanel from './UserPanel.svelte';
  import MessageComposer from '$lib/components/messages/MessageComposer.svelte';
</script>

<div class={ui.AppViewport}>
  <div class={ui.AppShell}>
    <Header />
    <Sidebar />
    <ChannelPanel />
    <ChatPanel />
    <UserPanel />
    <MessageComposer />
  </div>
</div>
