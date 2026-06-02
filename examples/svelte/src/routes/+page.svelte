<script lang="ts">
  import { ui } from '$lib/frame/generated';
  import '$lib/frame/generated.css';
</script>

<div class={ui.AppShell}>
  <aside class={ui.Sidebar}>Channels</aside>

  <main class={ui.Content}>
    <a class={ui.InlineLink} href="/inline-frame">Inline Frame route</a>

    <div class={ui.QuickLinks}>
      <a class={ui.QuickLinkCard} href="https://github.com/ThatOneToast/frame">Docs</a>
      <a class={ui.QuickLinkCard} href="https://github.com/ThatOneToast/frame">GitHub</a>
    </div>
  </main>

  <section class={ui.Inspector}>Details</section>
</div>
