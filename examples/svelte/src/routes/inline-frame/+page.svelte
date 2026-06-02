<main class="fr-PageShell">
  <section class="fr-HeroCard">
    <p class="fr-Eyebrow">Inline Frame</p>
    <h1>Frame Inline Style</h1>
    <p>This component uses style lang="frame".</p>
  </section>
</main>

<style lang="frame">
  stack PageShell {
    gap large
    padding large
    surface main
    height screen
  }

  card HeroCard {
    surface gradient dusk
    padding large
    radius large
    shadow medium

    hover {
      lift small
      glow accent
    }
  }

  text Eyebrow {
    text accent
    size caption
    weight semibold
  }
</style>
