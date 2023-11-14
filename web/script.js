window.onload = async () => {
    const canvas = document.querySelector(".preview");
    const last_update = document.querySelector(".last_update");
    const aliveness = document.querySelector(".aliveness");

    const ctx = canvas.getContext("2d");
    ctx.fillStyle = "#323232";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    let alive = true;
    
    setInterval(async () => {
        let response;
        try {
            response = await fetch("/planet");
            alive = true;
        } catch (e) {
            if (alive) {
                console.log("server doid");
                alive = false;
            }
        }

        aliveness.innerText = alive ? "alive" : "doid";

        if(!alive) {
            return;
        }
        
        let json_resonse = await response.json();

        let board = json_resonse.board;
        let planet_size = json_resonse.planet_size;

        canvas.width = planet_size;
        canvas.height = planet_size;

        for (let y = 0; y < planet_size; y++) {
            for (let x = 0; x < planet_size; x++) {
                let index = (x + (y * planet_size)) * 3;

                ctx.fillStyle = `rgb(${board[index + 0]}, ${board[index + 1]}, ${board[index + 2]})`;
                ctx.fillRect(x, y, 1, 1);
            }
        }

        last_update.innerText = new Date(Date.now()).toLocaleString('sv-SE', { timeZone: 'CET' });
    }, 1000);
}