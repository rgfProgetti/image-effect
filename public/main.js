async function init(){
    let rustApp = null

    try{
        rustApp = await import('../pkg')
    }catch(e){
        console.error(e)
        return;
    }


    const input = document.getElementById('upload')
    const loader = document.getElementById('loader')
    const uploadLabel = document.getElementById('upload-label')


    const fileReader = new FileReader()


    fileReader.onloadend = () => {
        const base64 = fileReader.result.replace(
            /^data:image\/(png|jpeg|pjg);base64,/,''
        )
        let img_data_url = rustApp.grayscale(base64)
        document.getElementById('new-img').setAttribute(
            'src',img_data_url
        )
        loader.classList.add(["hidden"])
        uploadLabel.classList.remove(["hidden"])
    }


    input.addEventListener('change', () =>{

        loader.classList.remove(["hidden"])
        uploadLabel.classList.add(["hidden"])
        document.getElementById('new-img').setAttribute(
            'src',''
        )
        fileReader.readAsDataURL(input.files[0])

        
    })
}

init()