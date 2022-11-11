const { invoke } = window.__TAURI__.tauri

let iptLength
let ckCapital
let ckSimple
let ckSpecial
let ckNumbers
let btnGenerate
let txtDisplay
let sliderDisplay
let containerDisplay

async function generatePassword() {
  console.log(iptLength.value)
  const config = {
    length: Number(iptLength?.value),
    useCapital: ckCapital?.checked,
    useSimple: ckSimple?.checked,
    useSpecial: ckSpecial?.checked,
    useNumber: ckNumbers?.checked,
  }

  console.log({ config })
  try {
    txtDisplay.textContent = await invoke('generate_password', {
      config,
    })
  } catch (error) {
    console.log(error)
  }
}

function applyDisabledButton() {
  if (
    !ckCapital.checked &&
    !ckSimple.checked &&
    !ckSpecial.checked &&
    !ckNumbers.checked
  ) {
    btnGenerate.disabled = true
  } else {
    btnGenerate.disabled = false
  }
}

function copyToClipboard() {
  const copyText = txtDisplay.textContent

  navigator.clipboard.writeText(copyText).then(
    function () {
      containerDisplay.classList.add('copied')
      setTimeout(() => {
        containerDisplay.classList.remove('copied')
      }, 1000)
    },
    function (err) {
      console.error('Async: Could not copy text: ', err)
    },
  )
}

window.addEventListener('DOMContentLoaded', () => {
  iptLength = document.querySelector('#ipt-length')
  ckCapital = document.querySelector('#use-capital')
  ckSimple = document.querySelector('#use-simple')
  ckSpecial = document.querySelector('#use-special')
  ckNumbers = document.querySelector('#use-number')
  btnGenerate = document.querySelector('#btn-generate')
  txtDisplay = document.querySelector('#txt-display')
  sliderDisplay = document.querySelector('#txt-slider')
  containerDisplay = document.querySelector('#container-display')

  sliderDisplay.textContent = iptLength.value

  iptLength.addEventListener(
    'change',
    (e) => (sliderDisplay.textContent = e.target.value),
  )
  ckCapital.addEventListener('change', applyDisabledButton)
  ckSimple.addEventListener('change', applyDisabledButton)
  ckSpecial.addEventListener('change', applyDisabledButton)
  ckNumbers.addEventListener('change', applyDisabledButton)
  btnGenerate.addEventListener('click', generatePassword)
  containerDisplay.addEventListener('click', copyToClipboard)
})
