<article>
    <header>
        <h1>Modpack Title</h1>
        <h2>1.2.3</h2>
    </header>

    <p>
        Last Played today <br />
        Last updated 5 days ago <br />
    </p>

    <footer>
        <button>Play</button>
        <button>Options</button>
    </footer>
</article>

<style lang="scss">
article {
    background: var(--primary-bg);
    border-radius: var(--rounding-large);

    p {
        padding: 1rem;
        margin: 0;
    }
}

header {
    padding-left: 1rem;
    padding-right: 1rem;
    padding-top: 1rem;
}

h1 {
   font-size: 2rem; 
   margin: 0;
}

h2 {
    margin-top: 0.25rem;
    margin-bottom: 0;
    color: color-mix(in srgb, var(--text) 70%, transparent);
}

footer {
    display: grid;
    grid-template-columns: 1fr 1fr;

    button {
        padding: 0.8rem 0;
        font-size: 1.2rem;
        background: var(--secondary-bg);
        color: var(--text);
        border: 0px solid var(--primary-bg);

        &:first-child {
            position: relative;
            border-radius: 0 0 0 var(--rounding-large);

            &::after {
                  content: "";
                  background: var(--text);
                  position: absolute;
                  bottom: 25%;
                  right: 0;
                  height: 50%;
                  width: 1px;
            }
        }

        &:last-child {
            border-radius: 0 0 var(--rounding-large) 0;
        }
    }
}
</style>
