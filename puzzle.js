const ROWS = 3;
const COLS = 3;

let targetTile; // blank

let turns = 0;

let imgVec = [1, 2, 3, 4, 5, 6, 7, 9, 8];
/*
 TODO: 
    - Need to know when its solved 
    - User supplied pictures
    - Variable amounts of rows and cols
*/
window.onload = () => {
    let _imgVec = [...imgVec];
    for (let i = 0; i < ROWS; i++) {
        for (let j = 0; j < COLS; j++) {
            let tile = document.createElement("img");

            let img = _imgVec.shift();
            if (img != 9) {
                tile.src = "img/" + img + ".jpg";
            } else {
                targetTile = tile;
            }
            tile.id = "tile-" + i.toString() + "-" + j.toString();
            tile.setAttribute("draggable", false);

            tile.addEventListener("click", onTileClick);
            
            document.getElementById("game-board").append(tile);
        }
    }
}

let onTileClick = (e) => {
    e.preventDefault();
    if (validateMove(e.target.id)) {
        // swap src
        targetTile.src = e.target.getAttribute("src");
        e.target.src = "";
        targetTile = e.target;

        incTurns();
    } else {
        document.getElementById("game-board").classList.add("shake");
        setTimeout(function() {
            document.getElementById("game-board").classList.remove("shake");
        }, 500);
    }
}

let validateMove = (tileId) => {
    let [, tileX, tileY] = tileId.split("-").map(Number);
    let [, targetX, targetY] = targetTile.id.split("-").map(Number);

    let xValid = tileX === targetX - 1 || tileX === targetX + 1;
    let yValid = tileY === targetY - 1 || tileY === targetY + 1;

    return (tileX === targetX || tileY === targetY) && (xValid || yValid);
}

let incTurns = () => {
    document.getElementById("turns").innerText = ++turns;
}
