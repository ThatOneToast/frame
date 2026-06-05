<script lang="ts">
  import { ui } from '$lib/generated/generated';
  import { activeChannel, submitMessage } from '$lib/stores/chat';

  let draft = '';

  async function send(): Promise<void> {
    const body = draft;
    draft = '';
    await submitMessage(body);
  }

  function handleSubmit(event: SubmitEvent): void {
    event.preventDefault();
    void send();
  }
</script>

<form
  class={ui.MessageComposer}
  aria-label="Message composer"
  onsubmit={handleSubmit}
>
  <label class={ui.ComposerLabel} for="message-input">msg</label>
  <input
    id="message-input"
    class={ui.ComposerInput}
    bind:value={draft}
    placeholder={`Message #${$activeChannel?.name ?? 'channel'}`}
    autocomplete="off"
  />
  <button class={ui.SendButton} type="submit" disabled={!draft.trim()}>send</button>
</form>
