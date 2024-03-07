<html>
<head>
    <style>
       body {
            background-color: #ffcccc;
            text-align: center;
            font-family: Arial, sans-serif;
        }

        .container {
            padding: 20px;
        }

        h1 {
            color: #ff3366;
        }

        .heart {
            font-size: 50px;
            color: #ff3366;
            cursor: pointer;
        }

        #nao {
            font-size: 24px;
            color: #333;
            background-color: #ffcccc;
            border: none;
            cursor: pointer;
            position: absolute;
        }
    </style>
    </style>
</head>
<body>
    <p>Oi meu amor ❤️,</p>
    <p>Quero dizer que você ilumina minha vida como ninguém mais.</p>
    <p class="heart" onclick="mostrarResposta()">❤️</p>
    <div class="box">
        <p>Te amo, Aceita um presente misterioso?</p>
        <div class="buttons-container">
            <a class="button" href="https://www.youtube.com/watch?v=2VVb_3CxGO8">Sim</a>
            <button id="no">Não</button>
        </div>
    </div>

    <script>
        let button = document.getElementById('no');
        let height = window.innerHeight - 50;
        let width = window.innerWidth - 50;

        // Evento para dispositivos móveis (touchstart)
        button.addEventListener('touchstart', function (e) {
            e.preventDefault(); // Impede o comportamento padrão do toque (por exemplo, zoom)
            button.style.position = "absolute";
            button.style.top = Math.random() * height + "px";
            button.style.left = Math.random() * width + "px";
        });

        // Evento para desktop (mouseover)
        button.addEventListener('mouseover', function () {
            button.style.position = "absolute";
            button.style.top = Math.random() * height + "px";
            button.style.left = Math.random() * width + "px";
        });
    </script>
</body>
</html>
