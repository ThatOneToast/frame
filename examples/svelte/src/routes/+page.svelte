<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<div class={ui.AppShell}>
  <aside class={ui.Sidebar}>Channels</aside>

  <main class={ui.Content}>
    <div class={ui.QuickLinks}>
      <a class={ui.QuickLinkCard}>Docs</a>
      <a class={ui.QuickLinkCard}>GitHub</a>
    </div>
  </main>

  <section class={ui.Inspector}>Details</section>
</div>
