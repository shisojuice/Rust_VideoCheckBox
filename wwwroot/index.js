import init, { ascii_filter } from './rust_videocheckbox.js';

const video = document.getElementById('myVideo');

const normal_canvas = document.getElementById('normalCanvas');
const normal_ctx = normal_canvas.getContext('2d',{willReadFrequently: true,});

navigator.mediaDevices.getUserMedia({ video: true, audio: false })
    .then(stream => {
        video.srcObject = stream;
        video.play();
        // 描画を開始
        video.addEventListener('loadeddata', () => {
            normal_canvas.width = 640;
            normal_canvas.height = 480;
            normal_canvas.style.width = '640px';
            normal_canvas.style.height ='480px';
            function draw() {
                if (video.paused || video.ended) return;
                normal_ctx.drawImage(video, 0, 0, normal_canvas.width, normal_canvas.height);
                requestAnimationFrame(draw);
            }
            draw();
        });
    })
    .catch(err => {
        console.error('エラー:', err);
    });

async function run() {
    await init();
    document.getElementById("asciiStart").addEventListener("click", () => {
        function draw() {
            const imageData = normal_ctx.getImageData(0, 0, normal_canvas.width, normal_canvas.height);
            const ret = ascii_filter(new Uint8Array(imageData.data.buffer),normal_canvas.width,normal_canvas.height,10);
            console.log(ret)
            for(let i=0;i<ret.length;i++)
            {
                if( document.getElementById(`chk${i+1}`))
                {
                    document.getElementById(`chk${i+1}`).checked = ret[i];
                }
            }
            requestAnimationFrame(draw);
        }
        draw();
    });
}
run();
