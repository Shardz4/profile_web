
    export function request_fullscreen() {
        const el = document.documentElement;
        if (el.requestFullscreen) {
            el.requestFullscreen().catch(err => {
                console.warn("Fullscreen request rejected:", err);
            });
        }
    }
    export function exit_fullscreen() {
        if (document.exitFullscreen) {
            document.exitFullscreen().catch(err => {
                console.warn("Exit fullscreen failed:", err);
            });
        }
    }
    export function setup_fullscreen_click() {
        document.addEventListener('click', () => {
            request_fullscreen();
            const event = new KeyboardEvent('keydown', {
                key: 'Enter',
                code: 'Enter',
                keyCode: 13,
                which: 13,
                bubbles: true,
                cancelable: true
            });
            window.dispatchEvent(event);
            document.dispatchEvent(event);
        }, { once: true });
    }
    export function open_url_js(url) {
        window.open(url, '_blank');
    }
    export function apply_terminal_font() {
        const setFont = () => {
            const canvases = document.querySelectorAll('#terminal-container canvas');
            canvases.forEach(canvas => {
                const ctx = canvas.getContext('2d');
                if (ctx) {
                    ctx.font = "16px 'JetBrains Mono', 'Font Awesome 6 Free', 'Font Awesome 6 Brands', monospace";
                }
            });
        };
        setFont();
        setTimeout(setFont, 100);
        setTimeout(setFont, 500);
        setTimeout(setFont, 1000);
        window.addEventListener('resize', setFont);
    }
