<script lang="ts">
  import { ui } from '$lib/generated/generated';
  import type { Message } from '$lib/models/message';
  import type { User } from '$lib/models/user';

  export let message: Message;
  export let author: User | undefined;

  const timeFormatter = new Intl.DateTimeFormat('en', {
    hour: '2-digit',
    minute: '2-digit'
  });
</script>

<article class={ui.MessageItem} role="listitem">
  <div class={ui.MessageAvatar} aria-hidden="true">
    {(author?.displayName ?? '?').slice(0, 2).toUpperCase()}
  </div>

  <div class={ui.MessageBody}>
    <header class={ui.MessageMeta}>
      <span class={ui.MessageAuthor}>{author?.displayName ?? 'Unknown user'}</span>
      <time class={ui.MessageTime} datetime={message.sentAt}>{timeFormatter.format(new Date(message.sentAt))}</time>
      {#if message.edited}
        <span class={ui.MessageEdited}>edited</span>
      {/if}
    </header>

    <p class={ui.MessageText}>{message.body}</p>

    {#if message.reactions.length > 0}
      <div class={ui.ReactionRow} aria-label="Reactions">
        {#each message.reactions as reaction}
          <span class={ui.ReactionPill}>{reaction.label} {reaction.count}</span>
        {/each}
      </div>
    {/if}
  </div>
</article>
