jQuery(document).ready(function () {
  (function (jQuery) {
    // jQuery('.accordion > li:eq(0) a').addClass('active').next().slideDown();
    jQuery('.js-accordion-btn').on('click', function (j) {
      var dropDown = jQuery(this).closest('li').find('.js-accordion-content');
      jQuery(this).closest('.js-accordion-container').find('.js-accordion-content').not(dropDown).slideUp();

      if (jQuery(this).hasClass('active')) {
        jQuery(this).removeClass('active');
        jQuery(this).closest('li').removeClass('active');
      } else {
        jQuery(this).closest('.js-accordion-container').find('.js-accordion-btn.active').removeClass('active');
        jQuery(this).addClass('active');
        jQuery('.js-accordion-container').find('li').removeClass('active');
        jQuery(this).closest('li').addClass('active');
      }

      dropDown.stop(false, true).slideToggle();
      j.preventDefault();
    });
  })(jQuery);
});
// CÃ³digo para criar conversor de linhas
// Universidade LusÃ³fona, DEISI, 2022
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Pesquisa de linha por municÃ­pio
function converteDeMunicipioLinhaAntiga() {
  //-------------------------------------------------------------------------------
  // Cria o seletor de municÃ­pios
  //-------------------------------------------------------------------------------
  let text = '<div>Selecione o MunicÃ­pio</div>';
  text += '<div>';
  text += '<select id="seletorMunicipios" class="nomePesquisa">';
  text += '<option value="escolha">escolha</option>'; // escolha

  for (let municipio in diretorio) {
    text += `<option value="${municipio}">${municipio}</option>`;
  }

  text += '</select>';
  text += '</div>';
  document.getElementById("divSeletorMunicipios").innerHTML = text;

  document.querySelector("#seletorMunicipios").onchange = function () {
    offsetId('pesquisaPorMunicipio'); // fazer desaparecer o resto 

    document.querySelector("input").value = '';
    document.getElementById("seletorOperadores").selectedIndex = 0;
    document.querySelector("#linhasConvertidasOperador").innerHTML = '';
    document.getElementById("listaNovasLinhas").innerHTML = '';
    document.getElementById("seletorNovasLinhasMunicipio").selectedIndex = 0;
    document.getElementById('carreiras').innerHTML = '';
    document.querySelector("#divOperadoresLinhas").innerHTML = ''; //-------------------------------------------------------------------------------
    // Cria o seletor de linhas do municipio 
    //-------------------------------------------------------------------------------

    let municipio = document.querySelector("#seletorMunicipios").value;
    let text = '<div>Linha antiga</div><select id="seletorLinhasMunicipio" class="numLinhaPesquisa"><option value="">nÂº</option>'; // escolha
    // for (const carreira in diretorio[municipio]) {
    //     if (carreira != 'Nova') {
    //         text += `<option value="${carreira}">${carreira}</option>`;
    //     }
    // }

    const lista = [];

    for (let carreira in diretorio[municipio]) {
      if (carreira != 'Nova') {
        lista.push(carreira);
      }
    }

    lista.sort();

    for (let carreira in lista) {
      text += `<option value="${lista[carreira]}">${lista[carreira]}</option>`;
    }

    text += '</select>';
    document.getElementById("divMunicipiosLinhas").innerHTML = text;
  }; //-------------------------------------------------------------------------------
  // Lista novos numeros duma linha antiga  
  //-------------------------------------------------------------------------------


  document.querySelector("#divMunicipiosLinhas").onchange = function () {
    let linha = document.querySelector("#seletorLinhasMunicipio").value;
    let municipio = document.querySelector("#seletorMunicipios").value;
    let text;
    if (diretorio[municipio][linha].length == 1) text = '<b>Nova linha:</b><br>';else text = '<b>Novas linhas:</b><br>';

    for (let i in diretorio[municipio][linha]) {
      let num = diretorio[municipio][linha][i][0];
      let nome = diretorio[municipio][linha][i][1];
      text += criaElementoLinhaComInfo(nome, num);
    }

    document.getElementById("linhasConvertidasMunicipio").innerHTML = text; // ativa evento de click no preferido, adicionando a lista de preferidos

    document.querySelectorAll('.novaLinha button.preferida').forEach(button => {
      button.onclick = () => {
        adiciona_favoritos(button);
        linhasPreferidas();
      };
    }); // ativa evento de click no horario, abrindo o horario

    document.querySelectorAll('.novaLinha button.verHorario, .novaLinha button.novaLinhaNum').forEach(button => {
      button.onclick = () => {
        offsetHorario();
        document.querySelector('#inputLinha').value = button.value;
        mostraLinha(); // document.querySelector('#carreirasPorMunicipioContent').style.display= 'none';
        // document.querySelector('#carreirasPorMunicipio').setAttribute('class', 'accordion__btn js-accordion-btn');
        // document.querySelector('#carreirasPorMunicipio').parentElement.setAttribute('class', '');

        document.querySelector('#horariosContent').style.display = 'block';
        document.querySelector('#horariosSeparador').setAttribute('class', 'accordion__btn js-accordion-btn active');
        document.querySelector('#horariosSeparador').parentElement.setAttribute('class', 'active');
      };
    });
  };
} //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Pesquisa de linha por operador
///////////////////////////////////////////////////////////


function converteDeOperadorLinhaAntiga() {
  //-------------------------------------------------------------------------------
  // Cria o seletor de operadores
  //-------------------------------------------------------------------------------
  let text = '<div>Selecione o Operador</div>';
  text += '<div>';
  text += '<select id="seletorOperadores" class="nomePesquisa">';
  text += '<option value="escolha">escolha</option>'; // restringir altura lista select:  onfocus="this.size=5;" onblur="this.size=1;" onchange="this.size=1; this.blur();"

  for (let operador in diretorio_operadores) {
    if (operador != 'Nova') text += `<option value="${operador}">${operador}</option>`;
  }

  text += '</select>';
  text += '</div>';
  document.getElementById("divSeletorOperadores").innerHTML = text;

  document.querySelector("#seletorOperadores").onchange = function () {
    offsetId('pesquisaPorOperador'); // fazer desaparecer o resto 

    document.querySelector("input").value = '';
    document.getElementById("seletorMunicipios").selectedIndex = 0;
    document.querySelector("#linhasConvertidasMunicipio").innerHTML = '';
    document.getElementById("listaNovasLinhas").innerHTML = '';
    document.querySelector("#divMunicipiosLinhas").innerHTML = '';
    document.getElementById("seletorNovasLinhasMunicipio").selectedIndex = 0;
    document.getElementById('carreiras').innerHTML = ''; //-------------------------------------------------------------------------------
    // Cria o seletor de linhas do operador 
    //-------------------------------------------------------------------------------

    document.getElementById("linhasConvertidasOperador").innerHTML = '';
    document.getElementById("divOperadoresLinhas").innerHTML = '';
    let operador = document.querySelector("#seletorOperadores").value;
    let text = '<div>Linha antiga</div><select id="seletorLinhasOperador" class="numLinhaPesquisa"><option value="">escolha</option>';
    const lista = [];

    for (let carreira in diretorio_operadores[operador]) {
      if (carreira != 'Nova') {
        lista.push(carreira);
      }
    }

    lista.sort();

    for (let carreira in lista) {
      text += `<option value="${lista[carreira]}">${lista[carreira]}</option>`;
    }

    text += '</select>';
    document.getElementById("divOperadoresLinhas").innerHTML = text;
  }; //-------------------------------------------------------------------------------
  // Lista novos numeros duma linha antiga  
  //-------------------------------------------------------------------------------


  document.querySelector("#divOperadoresLinhas").onchange = function () {
    let linha = document.querySelector("#seletorLinhasOperador").value;
    let operador = document.querySelector("#seletorOperadores").value;
    let text;
    if (diretorio_operadores[operador][linha].length == 1) text = '<b>Nova linha</b><br>';else text = '<b>Novas linhas:</b><br>';

    for (let i in diretorio_operadores[operador][linha]) {
      let num = diretorio_operadores[operador][linha][i][0];
      let nome = diretorio_operadores[operador][linha][i][1];
      text += criaElementoLinhaComInfo(nome, num);
    }

    document.getElementById("linhasConvertidasOperador").innerHTML = text; // ativa evento de click no preferido, adicionando a lista de preferidos

    document.querySelectorAll('.novaLinha button.preferida').forEach(button => {
      button.onclick = () => {
        adiciona_favoritos(button);
        linhasPreferidas();
      };
    }); // ativa evento de click no horario, abrindo o horario

    document.querySelectorAll('.novaLinha button.verHorario, .novaLinha button.novaLinhaNum').forEach(button => {
      button.onclick = () => {
        document.querySelector('#inputLinha').value = button.value;
        mostraLinha();
        offsetHorario(); // document.querySelector('#novasCarreirasPorMunicipioContent').style.display= 'none';
        // document.querySelector('#novasCarreirasPorMunicipio').setAttribute('class', 'accordion__btn js-accordion-btn');
        // document.querySelector('#novasCarreirasPorMunicipio').parentElement.setAttribute('class', '');

        document.querySelector('#horariosContent').style.display = 'block';
        document.querySelector('#horariosSeparador').setAttribute('class', 'accordion__btn js-accordion-btn active');
        document.querySelector('#horariosSeparador').parentElement.setAttribute('class', 'active');
      };
    });
  };
} ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Cria lista das Novas Linhas De um Municipio 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////


function listaNovasLinhasDeMunicipio() {
  // Cria seletor de municipios ------------------------------------------------------------------------------
  let text = '<div class="opcoesPesquisa"><div>';
  text += '<div>Selecione o MunicÃ­pio</div>';
  text += '<select id="seletorNovasLinhasMunicipio" class="nomePesquisa">';
  text += '<option value="escolha">escolha</option>';
  linhas = [];

  for (let municipio in diretorio) {
    // for (let velhaLinha in diretorio[municipio]) {
    //     if (diretorio[municipio][velhaLinha] != 'Nova');
    // }
    if ('Nova' in diretorio[municipio]) {
      text += `<option value="${municipio}">${municipio}</option>`;
    }
  }

  text += '</select></div></div>';
  text += '<div id="listaNovasLinhas"></div>';
  document.querySelector("#novasLinhasMunicipio").innerHTML = text; // Cria lista de novas linhas ------------------------------------------------------------------------------

  document.querySelector("#seletorNovasLinhasMunicipio").onchange = function () {
    // fecha tudo
    document.getElementById("seletorOperadores").selectedIndex = 0;
    document.querySelector("#linhasConvertidasOperador").innerHTML = '';
    document.getElementById("listaNovasLinhas").innerHTML = '';
    document.querySelector("#divOperadoresLinhas").innerHTML = '';
    document.getElementById("seletorMunicipios").selectedIndex = 0;
    document.querySelector("#linhasConvertidasMunicipio").innerHTML = '';
    document.getElementById("listaNovasLinhas").innerHTML = '';
    document.querySelector("#divMunicipiosLinhas").innerHTML = '';
    offsetId('novasCarreirasPorMunicipio');
    let municipio = document.querySelector("#seletorNovasLinhasMunicipio").value;
    let text = '';
    if (Object.keys(diretorio[municipio]['Nova']).length == 1) text = '<b>Nova linha</b><br>';else text = '<b>Novas linhas:</b><br>';

    for (let i in diretorio[municipio]['Nova']) {
      let num = diretorio[municipio]['Nova'][i][0];
      let nome = diretorio[municipio]['Nova'][i][1];
      text += criaElementoLinhaComInfo(nome, num);
    }

    if ('Nova' in diretorio[municipio] && diretorio[municipio]['Nova'].length > 0) {
      document.getElementById("listaNovasLinhas").innerHTML = text; // ativa evento de click no preferido, adicionando a lista de preferidos

      document.querySelectorAll('.novaLinha button.preferida').forEach(button => {
        button.onclick = () => {
          adiciona_favoritos(button);
          linhasPreferidas();
        };
      }); // ativa evento de click no horario, abrindo o horario

      document.querySelectorAll('.novaLinha button.verHorario, .novaLinha button.novaLinhaNum').forEach(button => {
        button.onclick = () => {
          document.querySelector('#inputLinha').value = button.value;
          mostraLinha();
          offsetHorario(); // document.querySelector('#novasCarreirasPorMunicipioContent').style.display= 'none';
          // document.querySelector('#novasCarreirasPorMunicipio').setAttribute('class', 'accordion__btn js-accordion-btn');
          // document.querySelector('#novasCarreirasPorMunicipio').parentElement.setAttribute('class', '');

          document.querySelector('#horariosContent').style.display = 'block';
          document.querySelector('#horariosSeparador').setAttribute('class', 'accordion__btn js-accordion-btn active');
          document.querySelector('#horariosSeparador').parentElement.setAttribute('class', 'active');
        };
      });
    }
  };
} ////////////////////////////////////////////////////////////////////////////////////////------------------------
// Consulte as linhas por MunicÃ­pio e veja seus horÃ¡rios
////////////////////////////////////////////////////////////////////


function pesquisaMunicipio() {
  var m = document.getElementById('meuMunicipio').value;
  meuMunicipio = m.charAt(0).toUpperCase() + m.slice(1).toLowerCase();

  if (meuMunicipio in diretorio) {
    document.getElementById('municipio').innerHTML = meuMunicipio + ' Ã© um municipio do diretorio';
  } else {
    document.getElementById('municipio').innerHTML = ' ';
  }
} ////////////////////////////////////////////////////////////////////////////////////////


function pesquisaNovaCarreira() {
  // fecha tudo
  // document.getElementById("seletorOperadores").selectedIndex = 0;
  // document.querySelector("#linhasConvertidasOperador").innerHTML = '';
  // document.getElementById("listaNovasLinhas").innerHTML = '';
  // document.querySelector("#divOperadoresLinhas").innerHTML = '';
  // document.getElementById("seletorMunicipios").selectedIndex = 0;
  // document.querySelector("#linhasConvertidasMunicipio").innerHTML = '';
  // document.getElementById("listaNovasLinhas").innerHTML = '';
  // document.querySelector("#divMunicipiosLinhas").innerHTML = '';
  var carreira_a_pesquisar = document.getElementById('minhaCarreira').value;
  var subDiretorio = {};

  for (let municipio in diretorio) {
    if (carreira_a_pesquisar in diretorio[municipio]) {
      for (let carreira in diretorio[municipio][carreira_a_pesquisar]) {
        c = diretorio[municipio][carreira_a_pesquisar][carreira][0];
        d = diretorio[municipio][carreira_a_pesquisar][carreira][1];
        subDiretorio[c] = d;
      }
    }
  }

  var tamanho = Object.keys(subDiretorio).length;

  if (tamanho == 0) {
    text = "";
  } else {
    var text;

    if (tamanho == 1) {
      text = "<b>Nova linha:</b><br>";
    } else {
      text = "<b>Novas linhas::</b><br>";
    }

    for (let carreira in subDiretorio) {
      text += criaElementoLinhaComInfo(subDiretorio[carreira], carreira);
    }
  }

  document.getElementById('carreiras').innerHTML = text; // document.getElementById("linhasPreferidasLista").style.display = 'block';
  // ativa evento de click no preferido, adicionando a lista de preferidos

  document.querySelectorAll('.novaLinha button.preferida').forEach(button => {
    button.onclick = () => {
      adiciona_favoritos(button);
      linhasPreferidas();
    };
  }); // ativa evento de click no horario, abrindo o horario

  document.querySelectorAll('.novaLinha button.verHorario, .novaLinha button.novaLinhaNum').forEach(button => {
    button.onclick = () => {
      document.querySelector('#inputLinha').value = button.value;
      mostraLinha();
      offsetHorario(); // document.querySelector('#novasCarreirasPorMunicipioContent').style.display= 'none';
      // document.querySelector('#novasCarreirasPorMunicipio').setAttribute('class', 'accordion__btn js-accordion-btn');
      // document.querySelector('#novasCarreirasPorMunicipio').parentElement.setAttribute('class', '');

      document.querySelector('#horariosContent').style.display = 'block';
      document.querySelector('#horariosSeparador').setAttribute('class', 'accordion__btn js-accordion-btn active');
      document.querySelector('#horariosSeparador').parentElement.setAttribute('class', 'active');
    };
  });
}

function da_cor(linha) {
  var cor;
  var tarifario = linhas_e_tarifario[linha];

  switch (tarifario) {
    case 'RÃ¡pida':
      cor = '#ffb005';
      break;

    case 'Longa':
      cor = '#ff0047';
      break;

    case 'Mar':
      cor = '#3dff9e';
      break;

    case 'PrÃ³xima':
      cor = '#4099ff';
      break;

    case 'Inter-regional':
      cor = '#bd1aff';
      break;

    case 'TurÃ­stica':
      cor = '#ff5900';
      break;
  }

  return cor;
} ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Cria lista de Linhas De um Municipio 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////


function listaLinhasDeMunicipio() {
  // Cria seletor com lista de municipios ------------------------------------------------------------------------------
  let text = '<div class="opcoesPesquisa"><div>';
  text += '<div>Selecione o MunicÃ­pio</div>';
  text += '<select id="idLinhaEscolhida" class="nomePesquisa">';
  text += '<option value="escolha">escolha</option>';
  linhas = [];

  for (let municipio in diretorio_linhas_por_municipio) {
    text += `<option value="${municipio}">${municipio}</option>`;
  }

  text += '</select></div></div>';
  text += '<div id="idListaLinhas" style="display:none"></div>';
  document.querySelector("#idLinhasMunicipio").innerHTML = text;

  document.querySelector("#CarreirasPorMunicipio").onclick = function () {
    document.querySelector('#horariosContent').style.display = 'none';
    document.querySelector('#horariosSeparador').setAttribute('class', 'accordion__btn js-accordion-btn');
    document.querySelector('#horariosSeparador').parentElement.setAttribute('class', ''); // offsetId('CarreirasPorMunicipio');

    offsetHorario();
  }; // Cria lista de novas linhas ------------------------------------------------------------------------------


  document.querySelector("#idLinhaEscolhida").onchange = function () {
    // fecha tudo
    // document.getElementById("seletorOperadores").selectedIndex = 0;
    // document.querySelector("#linhasConvertidasOperador").innerHTML = '';
    // document.getElementById("listaNovasLinhas").innerHTML = '';
    // document.querySelector("#divOperadoresLinhas").innerHTML = '';
    // document.getElementById("seletorMunicipios").selectedIndex = 0;
    // document.querySelector("#linhasConvertidasMunicipio").innerHTML = '';
    // document.getElementById("listaNovasLinhas").innerHTML = '';
    // document.querySelector("#divMunicipiosLinhas").innerHTML = '';
    offsetId('CarreirasPorMunicipio');
    document.querySelector('#legenda_tipos_linha_na_lista').style.display = 'block';
    let municipio = document.querySelector("#idLinhaEscolhida").value;
    var tamanho = Object.keys(diretorio_linhas_por_municipio).length; // insere frase antes da lista com scroll
    // if (tamanho == 1) {
    //     text = "<b>Clique no nÃºmero para ver o percurso e horÃ¡rio:</b><br>";
    // } else {
    //     text = "<b>Clique no nÃºmero para ver o percurso e horÃ¡rio:</b><br>";
    // }
    // let comentario = document.createElement('div');
    // comentario.innerHTML = text;
    // let pai = document.querySelector('#idLinhasMunicipio');
    // let lista = document.querySelector("#idListaLinhas");
    // pai.insertBefore(comentario, lista);

    text = '';

    for (let num in diretorio_linhas_por_municipio[municipio]) {
      let nome = diretorio_linhas_por_municipio[municipio][num][0];
      text += criaElementoLinhaComInfo(nome, num);
    }

    document.getElementById("idListaLinhas").innerHTML = text;
    document.getElementById("idListaLinhas").style.display = 'block'; // ativa evento de click no preferido, adicionando a lista de preferidos

    document.querySelectorAll('.novaLinha button.preferida').forEach(button => {
      button.onclick = () => {
        adiciona_favoritos(button);
        linhasPreferidas();
      };
    }); // ativa evento de click no horario, abrindo o horario

    document.querySelectorAll('.novaLinha button.verHorario, .novaLinha button.novaLinhaNum').forEach(button => {
      button.onclick = () => {
        offsetHorario();
        document.querySelector('#inputLinha').value = button.value;
        mostraLinha();
        document.querySelector('#CarreirasPorMunicipioContent').style.display = 'none';
        document.querySelector('#CarreirasPorMunicipio').setAttribute('class', 'accordion__btn js-accordion-btn');
        document.querySelector('#CarreirasPorMunicipio').parentElement.setAttribute('class', '');
        document.querySelector('#horariosContent').style.display = 'block';
        document.querySelector('#horariosSeparador').setAttribute('class', 'accordion__btn js-accordion-btn active');
        document.querySelector('#horariosSeparador').parentElement.setAttribute('class', 'active');
      };
    });
  };
} /////////////////////////////////////
// criaElementoLinhaComInfo
/////////////////////////////////////


function criaElementoLinhaComInfo(nome, num) {
  let text = `<div class='novaLinha'>`;
  text += `    <button class="novaLinhaNum"  value='${num}' style='background-color:${da_cor(num)}'>${num}</button>`;
  text += `    <div class="novaLinhaNome">${nome}</div>`;
  text += `    <button class="novaLinhaNum verHorario"  value='${num}' style='background-color:lightgrey'>HorÃ¡rio</button>`;

  if ('listaLinhasPreferidas' in localStorage && localStorage.getItem('listaLinhasPreferidas').includes(num)) {
    text += `    <button class="preferida" title="Adicionar Ã s minhas linhas favoritas" value='${num}' style='background-color:none; border:none;'><span class="material-symbols-outlined" style="color:#ffdd00">star</span></button>`;
    document.querySelectorAll(`novaLinha [value='${num}]`).forEach(button => {
      button.querySelector('.material-symbols-outlined').style.color = '#ffdd00';
    });
  } else {
    text += `    <button class="preferida" title="Adicionar Ã s minhas linhas favoritas" value='${num}' style='background-color:none; border:none;'><span class="material-symbols-outlined" style="color:lightgrey">star</span></button>`;
    document.querySelectorAll(`novaLinha [value='${num}]`).forEach(button => {
      button.querySelector('.material-symbols-outlined').style.color = 'lightgrey';
    });
  }

  text += `</div>`;
  return text;
} /////////////////////////////////////
// adiciona_favoritos
/////////////////////////////////////


function adiciona_favoritos(button) {
  document.querySelector('#separadorPreferidas').style.display = 'block';

  if (!localStorage.getItem('listaLinhasPreferidas')) {
    // nÃ£o existe lista de preferidas, adiciona
    let lista = [];
    lista.push(button.value);
    localStorage.setItem('listaLinhasPreferidas', JSON.stringify(lista));
    button.querySelector('span').style.color = '#ffdd00';
    document.querySelector('#separadorPreferidas').style.display = 'block';
  } else {
    // existe lista
    let lista_preferidas = JSON.parse(localStorage.getItem('listaLinhasPreferidas'));
    let linha = button.value;
    const index = lista_preferidas.indexOf(linha);

    if (index > -1) {
      // linha existe, sendo removida
      lista_preferidas.splice(index, 1);
      lista_preferidas.sort();
      localStorage.setItem('listaLinhasPreferidas', JSON.stringify(lista_preferidas)); // muda a cor da estrela, assim como de todas as estrelas dessa linha q estejam renderizadas na pagina

      button.querySelector('span').style.color = 'lightgrey';
      document.querySelector(`#linhaPreferida[value='${linha}'] span`);
      let estrela = document.querySelector(`#linhaPreferida[value='${linha}'] span`);

      if (estrela != null) {
        estrela.style.color = 'lightgrey';
      }

      document.querySelectorAll(`.preferida[value='${linha}'] span`).forEach(span => {
        span.style.color = 'lightgrey';
      });

      if (localStorage.getItem('listaLinhasPreferidas') == '[]') {
        // se lista ficar vazia, Ã© removida do localstorage 
        localStorage.removeItem('listaLinhasPreferidas');
        document.querySelector('#separadorPreferidas').style.display = 'none';
      }
    } else {
      // linha nÃ£o existe nos preferidos, sendo adicionada
      lista_preferidas.push(linha);
      lista_preferidas.sort();
      localStorage.setItem('listaLinhasPreferidas', JSON.stringify(lista_preferidas)); // muda a cor da estrela, assim como de todas as estrelas dessa linha q estejam renderizadas na pagina

      button.querySelector('span').style.color = '#ffdd00';
      let estrela = document.querySelector(`#linhaPreferida[value='${linha}'] span`);

      if (estrela != null) {
        estrela.style.color = '#ffdd00';
      }

      document.querySelectorAll(`.preferida[value='${linha}'] span`).forEach(span => {
        span.style.color = '#ffdd00';
      });
    }

    linhasPreferidas();
  }
} /////////////////////////////////////
// Carreiras preferidas
/////////////////////////////////////


function linhasPreferidas() {
  if (localStorage.getItem('listaLinhasPreferidas')) {
    const lista_preferidos = JSON.parse(localStorage.getItem('listaLinhasPreferidas'));
    let text = '';

    for (let i in lista_preferidos) {
      let num = lista_preferidos[i];
      let nome = diretorio_linhas[num][0];
      let linha = criaElementoLinhaComInfo(nome, num);
      linha = linha.replace('star', 'delete_forever');
      linha = linha.replace('#ffdd00', 'red');
      linha = linha.replace('Adicionar Ã s minhas linhas favoritas', 'Remover das favoritas');
      text += linha;
    }

    document.getElementById("linhasPreferidasLista").innerHTML = text;
    document.getElementById("linhasPreferidasLista").style.display = 'block'; // ativa evento de click no preferido, adicionando a lista de preferidos

    document.querySelectorAll('.novaLinha button.preferida').forEach(button => {
      button.onclick = () => {
        adiciona_favoritos(button);
        linhasPreferidas();
      };
    }); // caso clique no numero da linha ou horario, abre horario

    document.querySelectorAll('.novaLinha button.verHorario, .novaLinha button.novaLinhaNum').forEach(button => {
      button.onclick = () => {
        offsetHorario();
        document.querySelector('#inputLinha').value = button.value;
        mostraLinha(); // document.querySelector('#linhasPreferidasLista').style.display= 'none';
        // document.querySelector('#separadorPreferidas').setAttribute('class', '');
        // document.querySelector('#linhasPreferidasContent').setAttribute('class', 'accordion__btn js-accordion-btn');
        // document.querySelector('#linhasPreferidasLista').parentElement.setAttribute('class', '');

        document.querySelector('#horariosContent').style.display = 'block';
        document.querySelector('#horariosSeparador').setAttribute('class', 'accordion__btn js-accordion-btn active');
        document.querySelector('#horariosSeparador').parentElement.setAttribute('class', 'active');
      };
    });
  }
} ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Listener 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////


document.addEventListener('DOMContentLoaded', () => {
  converteDeMunicipioLinhaAntiga();
  converteDeOperadorLinhaAntiga();
  listaNovasLinhasDeMunicipio();
  listaLinhasDeMunicipio();
  linhasPreferidas();
});
const data_periodo_dia = {
  "20220101": ["2", "3"],
  "20220102": ["2", "3"],
  "20220103": ["1", "1"],
  "20220104": ["1", "1"],
  "20220105": ["1", "1"],
  "20220106": ["1", "1"],
  "20220107": ["1", "1"],
  "20220108": ["1", "2"],
  "20220109": ["1", "3"],
  "20220110": ["1", "1"],
  "20220111": ["1", "1"],
  "20220112": ["1", "1"],
  "20220113": ["1", "1"],
  "20220114": ["1", "1"],
  "20220115": ["1", "2"],
  "20220116": ["1", "3"],
  "20220117": ["1", "1"],
  "20220118": ["1", "1"],
  "20220119": ["1", "1"],
  "20220120": ["1", "1"],
  "20220121": ["1", "1"],
  "20220122": ["1", "2"],
  "20220123": ["1", "3"],
  "20220124": ["1", "1"],
  "20220125": ["1", "1"],
  "20220126": ["1", "1"],
  "20220127": ["1", "1"],
  "20220128": ["1", "1"],
  "20220129": ["1", "2"],
  "20220130": ["1", "3"],
  "20220131": ["1", "1"],
  "20220201": ["1", "1"],
  "20220202": ["1", "1"],
  "20220203": ["1", "1"],
  "20220204": ["1", "1"],
  "20220205": ["1", "2"],
  "20220206": ["1", "3"],
  "20220207": ["1", "1"],
  "20220208": ["1", "1"],
  "20220209": ["1", "1"],
  "20220210": ["1", "1"],
  "20220211": ["1", "1"],
  "20220212": ["1", "2"],
  "20220213": ["1", "3"],
  "20220214": ["1", "1"],
  "20220215": ["1", "1"],
  "20220216": ["1", "1"],
  "20220217": ["1", "1"],
  "20220218": ["1", "1"],
  "20220219": ["1", "2"],
  "20220220": ["1", "3"],
  "20220221": ["1", "1"],
  "20220222": ["1", "1"],
  "20220223": ["1", "1"],
  "20220224": ["1", "1"],
  "20220225": ["1", "1"],
  "20220226": ["1", "2"],
  "20220227": ["1", "3"],
  "20220228": ["1", "1"],
  "20220301": ["1", "1"],
  "20220302": ["1", "1"],
  "20220303": ["1", "1"],
  "20220304": ["1", "1"],
  "20220305": ["1", "2"],
  "20220306": ["1", "3"],
  "20220307": ["1", "1"],
  "20220308": ["1", "1"],
  "20220309": ["1", "1"],
  "20220310": ["1", "1"],
  "20220311": ["1", "1"],
  "20220312": ["1", "2"],
  "20220313": ["1", "3"],
  "20220314": ["1", "1"],
  "20220315": ["1", "1"],
  "20220316": ["1", "1"],
  "20220317": ["1", "1"],
  "20220318": ["1", "1"],
  "20220319": ["1", "2"],
  "20220320": ["1", "3"],
  "20220321": ["1", "1"],
  "20220322": ["1", "1"],
  "20220323": ["1", "1"],
  "20220324": ["1", "1"],
  "20220325": ["1", "1"],
  "20220326": ["1", "2"],
  "20220327": ["1", "3"],
  "20220328": ["1", "1"],
  "20220329": ["1", "1"],
  "20220330": ["1", "1"],
  "20220331": ["1", "1"],
  "20220401": ["1", "1"],
  "20220402": ["1", "2"],
  "20220403": ["1", "3"],
  "20220404": ["1", "1"],
  "20220405": ["1", "1"],
  "20220406": ["1", "1"],
  "20220407": ["1", "1"],
  "20220408": ["2", "1"],
  "20220409": ["2", "2"],
  "20220410": ["2", "3"],
  "20220411": ["2", "1"],
  "20220412": ["2", "1"],
  "20220413": ["2", "1"],
  "20220414": ["2", "1"],
  "20220415": ["2", "3"],
  "20220416": ["2", "2"],
  "20220417": ["2", "3"],
  "20220418": ["2", "1"],
  "20220419": ["2", "1"],
  "20220420": ["2", "1"],
  "20220421": ["2", "1"],
  "20220422": ["1", "1"],
  "20220423": ["1", "2"],
  "20220424": ["1", "3"],
  "20220425": ["1", "3"],
  "20220426": ["1", "1"],
  "20220427": ["1", "1"],
  "20220428": ["1", "1"],
  "20220429": ["1", "1"],
  "20220430": ["1", "2"],
  "20220501": ["1", "3"],
  "20220502": ["1", "1"],
  "20220503": ["1", "1"],
  "20220504": ["1", "1"],
  "20220505": ["1", "1"],
  "20220506": ["1", "1"],
  "20220507": ["1", "2"],
  "20220508": ["1", "3"],
  "20220509": ["1", "1"],
  "20220510": ["1", "1"],
  "20220511": ["1", "1"],
  "20220512": ["1", "1"],
  "20220513": ["1", "1"],
  "20220514": ["1", "2"],
  "20220515": ["1", "3"],
  "20220516": ["1", "1"],
  "20220517": ["1", "1"],
  "20220518": ["1", "1"],
  "20220519": ["1", "1"],
  "20220520": ["1", "1"],
  "20220521": ["1", "2"],
  "20220522": ["1", "3"],
  "20220523": ["1", "1"],
  "20220524": ["1", "1"],
  "20220525": ["1", "1"],
  "20220526": ["1", "1"],
  "20220527": ["1", "1"],
  "20220528": ["1", "2"],
  "20220529": ["1", "3"],
  "20220530": ["1", "1"],
  "20220531": ["1", "1"],
  "20220601": ["1", "1"],
  "20220602": ["1", "1"],
  "20220603": ["1", "1"],
  "20220604": ["1", "2"],
  "20220605": ["1", "3"],
  "20220606": ["1", "1"],
  "20220607": ["1", "1"],
  "20220608": ["1", "1"],
  "20220609": ["1", "1"],
  "20220610": ["1", "3"],
  "20220611": ["1", "2"],
  "20220612": ["1", "3"],
  "20220613": ["1", "1"],
  "20220614": ["1", "1"],
  "20220615": ["1", "1"],
  "20220616": ["1", "3"],
  "20220617": ["1", "1"],
  "20220618": ["1", "2"],
  "20220619": ["1", "3"],
  "20220620": ["1", "1"],
  "20220621": ["1", "1"],
  "20220622": ["1", "1"],
  "20220623": ["2", "1"],
  "20220624": ["2", "1"],
  "20220625": ["2", "2"],
  "20220626": ["2", "3"],
  "20220627": ["2", "1"],
  "20220628": ["2", "1"],
  "20220629": ["2", "1"],
  "20220630": ["2", "1"],
  "20220701": ["3", "1"],
  "20220702": ["3", "2"],
  "20220703": ["3", "3"],
  "20220704": ["3", "1"],
  "20220705": ["3", "1"],
  "20220706": ["3", "1"],
  "20220707": ["3", "1"],
  "20220708": ["3", "1"],
  "20220709": ["3", "2"],
  "20220710": ["3", "3"],
  "20220711": ["3", "1"],
  "20220712": ["3", "1"],
  "20220713": ["3", "1"],
  "20220714": ["3", "1"],
  "20220715": ["3", "1"],
  "20220716": ["3", "2"],
  "20220717": ["3", "3"],
  "20220718": ["3", "1"],
  "20220719": ["3", "1"],
  "20220720": ["3", "1"],
  "20220721": ["3", "1"],
  "20220722": ["3", "1"],
  "20220723": ["3", "2"],
  "20220724": ["3", "3"],
  "20220725": ["3", "1"],
  "20220726": ["3", "1"],
  "20220727": ["3", "1"],
  "20220728": ["3", "1"],
  "20220729": ["3", "1"],
  "20220730": ["3", "2"],
  "20220731": ["3", "3"],
  "20220801": ["3", "1"],
  "20220802": ["3", "1"],
  "20220803": ["3", "1"],
  "20220804": ["3", "1"],
  "20220805": ["3", "1"],
  "20220806": ["3", "2"],
  "20220807": ["3", "3"],
  "20220808": ["3", "1"],
  "20220809": ["3", "1"],
  "20220810": ["3", "1"],
  "20220811": ["3", "1"],
  "20220812": ["3", "1"],
  "20220813": ["3", "2"],
  "20220814": ["3", "3"],
  "20220815": ["3", "3"],
  "20220816": ["3", "1"],
  "20220817": ["3", "1"],
  "20220818": ["3", "1"],
  "20220819": ["3", "1"],
  "20220820": ["3", "2"],
  "20220821": ["3", "3"],
  "20220822": ["3", "1"],
  "20220823": ["3", "1"],
  "20220824": ["3", "1"],
  "20220825": ["3", "1"],
  "20220826": ["3", "1"],
  "20220827": ["3", "2"],
  "20220828": ["3", "3"],
  "20220829": ["3", "1"],
  "20220830": ["3", "1"],
  "20220831": ["3", "1"],
  "20220901": ["2", "1"],
  "20220902": ["2", "1"],
  "20220903": ["2", "2"],
  "20220904": ["2", "3"],
  "20220905": ["2", "1"],
  "20220906": ["2", "1"],
  "20220907": ["2", "1"],
  "20220908": ["2", "1"],
  "20220909": ["2", "1"],
  "20220910": ["2", "2"],
  "20220911": ["2", "3"],
  "20220912": ["2", "1"],
  "20220913": ["2", "1"],
  "20220914": ["2", "1"],
  "20220915": ["2", "1"],
  "20220916": ["1", "1"],
  "20220917": ["1", "2"],
  "20220918": ["1", "3"],
  "20220919": ["1", "1"],
  "20220920": ["1", "1"],
  "20220921": ["1", "1"],
  "20220922": ["1", "1"],
  "20220923": ["1", "1"],
  "20220924": ["1", "2"],
  "20220925": ["1", "3"],
  "20220926": ["1", "1"],
  "20220927": ["1", "1"],
  "20220928": ["1", "1"],
  "20220929": ["1", "1"],
  "20220930": ["1", "1"],
  "20221001": ["1", "2"],
  "20221002": ["1", "3"],
  "20221003": ["1", "1"],
  "20221004": ["1", "1"],
  "20221005": ["1", "3"],
  "20221006": ["1", "1"],
  "20221007": ["1", "1"],
  "20221008": ["1", "2"],
  "20221009": ["1", "3"],
  "20221010": ["1", "1"],
  "20221011": ["1", "1"],
  "20221012": ["1", "1"],
  "20221013": ["1", "1"],
  "20221014": ["1", "1"],
  "20221015": ["1", "2"],
  "20221016": ["1", "3"],
  "20221017": ["1", "1"],
  "20221018": ["1", "1"],
  "20221019": ["1", "1"],
  "20221020": ["1", "1"],
  "20221021": ["1", "1"],
  "20221022": ["1", "2"],
  "20221023": ["1", "3"],
  "20221024": ["1", "1"],
  "20221025": ["1", "1"],
  "20221026": ["1", "1"],
  "20221027": ["1", "1"],
  "20221028": ["1", "1"],
  "20221029": ["1", "2"],
  "20221030": ["1", "3"],
  "20221031": ["1", "1"],
  "20221101": ["1", "3"],
  "20221102": ["1", "1"],
  "20221103": ["1", "1"],
  "20221104": ["1", "1"],
  "20221105": ["1", "2"],
  "20221106": ["1", "3"],
  "20221107": ["1", "1"],
  "20221108": ["1", "1"],
  "20221109": ["1", "1"],
  "20221110": ["1", "1"],
  "20221111": ["1", "1"],
  "20221112": ["1", "2"],
  "20221113": ["1", "3"],
  "20221114": ["1", "1"],
  "20221115": ["1", "1"],
  "20221116": ["1", "1"],
  "20221117": ["1", "1"],
  "20221118": ["1", "1"],
  "20221119": ["1", "2"],
  "20221120": ["1", "3"],
  "20221121": ["1", "1"],
  "20221122": ["1", "1"],
  "20221123": ["1", "1"],
  "20221124": ["1", "1"],
  "20221125": ["1", "1"],
  "20221126": ["1", "2"],
  "20221127": ["1", "3"],
  "20221128": ["1", "1"],
  "20221129": ["1", "1"],
  "20221130": ["1", "1"],
  "20221201": ["1", "3"],
  "20221202": ["1", "1"],
  "20221203": ["1", "2"],
  "20221204": ["1", "3"],
  "20221205": ["1", "1"],
  "20221206": ["1", "1"],
  "20221207": ["1", "1"],
  "20221208": ["1", "3"],
  "20221209": ["1", "1"],
  "20221210": ["1", "2"],
  "20221211": ["1", "3"],
  "20221212": ["1", "1"],
  "20221213": ["1", "1"],
  "20221214": ["1", "1"],
  "20221215": ["1", "1"],
  "20221216": ["1", "1"],
  "20221217": ["2", "2"],
  "20221218": ["2", "3"],
  "20221219": ["2", "1"],
  "20221220": ["2", "1"],
  "20221221": ["2", "1"],
  "20221222": ["2", "1"],
  "20221223": ["2", "1"],
  "20221224": ["2", "2"],
  "20221225": ["2", "3"],
  "20221226": ["2", "1"],
  "20221227": ["2", "1"],
  "20221228": ["2", "1"],
  "20221229": ["2", "1"],
  "20221230": ["2", "1"],
  "20221231": ["2", "2"],
  "20230101": ["2", "3"],
  "20230102": ["2", "1"],
  "20230103": ["1", "1"],
  "20230104": ["1", "1"],
  "20230105": ["1", "1"],
  "20230106": ["1", "1"],
  "20230107": ["1", "2"],
  "20230108": ["1", "3"],
  "20230109": ["1", "1"],
  "20230110": ["1", "1"],
  "20230111": ["1", "1"],
  "20230112": ["1", "1"],
  "20230113": ["1", "1"],
  "20230114": ["1", "2"],
  "20230115": ["1", "3"],
  "20230116": ["1", "1"],
  "20230117": ["1", "1"],
  "20230118": ["1", "1"],
  "20230119": ["1", "1"],
  "20230120": ["1", "1"],
  "20230121": ["1", "2"],
  "20230122": ["1", "3"],
  "20230123": ["1", "1"],
  "20230124": ["1", "1"],
  "20230125": ["1", "1"],
  "20230126": ["1", "1"],
  "20230127": ["1", "1"],
  "20230128": ["1", "2"],
  "20230129": ["1", "3"],
  "20230130": ["1", "1"],
  "20230131": ["1", "1"],
  "20230201": ["1", "1"],
  "20230202": ["1", "1"],
  "20230203": ["1", "1"],
  "20230204": ["1", "2"],
  "20230205": ["1", "3"],
  "20230206": ["1", "1"],
  "20230207": ["1", "1"],
  "20230208": ["1", "1"],
  "20230209": ["1", "1"],
  "20230210": ["1", "1"],
  "20230211": ["1", "2"],
  "20230212": ["1", "3"],
  "20230213": ["1", "1"],
  "20230214": ["1", "1"],
  "20230215": ["1", "1"],
  "20230216": ["1", "1"],
  "20230217": ["1", "1"],
  "20230218": ["2", "2"],
  "20230219": ["2", "3"],
  "20230220": ["2", "1"],
  "20230221": ["2", "3"],
  "20230222": ["2", "1"],
  "20230223": ["1", "1"],
  "20230224": ["1", "1"],
  "20230225": ["1", "2"],
  "20230226": ["1", "3"],
  "20230227": ["1", "1"],
  "20230228": ["1", "1"],
  "20230301": ["1", "1"],
  "20230302": ["1", "1"],
  "20230303": ["1", "1"],
  "20230304": ["1", "2"],
  "20230305": ["1", "3"],
  "20230306": ["1", "1"],
  "20230307": ["1", "1"],
  "20230308": ["1", "1"],
  "20230309": ["1", "1"],
  "20230310": ["1", "1"],
  "20230311": ["1", "2"],
  "20230312": ["1", "3"],
  "20230313": ["1", "1"],
  "20230314": ["1", "1"],
  "20230315": ["1", "1"],
  "20230316": ["1", "1"],
  "20230317": ["1", "1"],
  "20230318": ["1", "2"],
  "20230319": ["1", "3"],
  "20230320": ["1", "1"],
  "20230321": ["1", "1"],
  "20230322": ["1", "1"],
  "20230323": ["1", "1"],
  "20230324": ["1", "1"],
  "20230325": ["2", "2"],
  "20230326": ["2", "3"],
  "20230327": ["2", "1"],
  "20230328": ["2", "1"],
  "20230329": ["2", "1"],
  "20230330": ["2", "1"],
  "20230331": ["2", "1"],
  "20230401": ["2", "2"],
  "20230402": ["2", "3"],
  "20230403": ["2", "1"],
  "20230404": ["2", "1"],
  "20230405": ["2", "1"],
  "20230406": ["2", "1"],
  "20230407": ["2", "3"],
  "20230408": ["2", "2"],
  "20230409": ["2", "3"],
  "20230410": ["1", "1"],
  "20230411": ["1", "1"],
  "20230412": ["1", "1"],
  "20230413": ["1", "1"],
  "20230414": ["1", "1"],
  "20230415": ["1", "2"],
  "20230416": ["1", "3"],
  "20230417": ["1", "1"],
  "20230418": ["1", "1"],
  "20230419": ["1", "1"],
  "20230420": ["1", "1"],
  "20230421": ["1", "1"],
  "20230422": ["1", "2"],
  "20230423": ["1", "3"],
  "20230424": ["1", "1"],
  "20230425": ["1", "3"],
  "20230426": ["1", "1"],
  "20230427": ["1", "1"],
  "20230428": ["1", "1"],
  "20230429": ["1", "2"],
  "20230430": ["1", "3"],
  "20230501": ["1", "3"],
  "20230502": ["1", "1"],
  "20230503": ["1", "1"],
  "20230504": ["1", "1"],
  "20230505": ["1", "1"],
  "20230506": ["1", "2"],
  "20230507": ["1", "3"],
  "20230508": ["1", "1"],
  "20230509": ["1", "1"],
  "20230510": ["1", "1"],
  "20230511": ["1", "1"],
  "20230512": ["1", "1"],
  "20230513": ["1", "2"],
  "20230514": ["1", "3"],
  "20230515": ["1", "1"],
  "20230516": ["1", "1"],
  "20230517": ["1", "1"],
  "20230518": ["1", "1"],
  "20230519": ["1", "1"],
  "20230520": ["1", "2"],
  "20230521": ["1", "3"],
  "20230522": ["1", "1"],
  "20230523": ["1", "1"],
  "20230524": ["1", "1"],
  "20230525": ["1", "1"],
  "20230526": ["1", "1"],
  "20230527": ["1", "2"],
  "20230528": ["1", "3"],
  "20230529": ["1", "1"],
  "20230530": ["1", "1"],
  "20230531": ["1", "1"],
  "20230601": ["1", "1"],
  "20230602": ["1", "1"],
  "20230603": ["1", "2"],
  "20230604": ["1", "3"],
  "20230605": ["1", "1"],
  "20230606": ["1", "1"],
  "20230607": ["1", "1"],
  "20230608": ["1", "3"],
  "20230609": ["1", "1"],
  "20230610": ["1", "3"],
  "20230611": ["1", "3"],
  "20230612": ["1", "1"],
  "20230613": ["1", "1"],
  "20230614": ["1", "1"],
  "20230615": ["1", "1"],
  "20230616": ["1", "1"],
  "20230617": ["1", "2"],
  "20230618": ["1", "3"],
  "20230619": ["2", "1"],
  "20230620": ["2", "1"],
  "20230621": ["2", "1"],
  "20230622": ["2", "1"],
  "20230623": ["2", "1"],
  "20230624": ["2", "2"],
  "20230625": ["2", "3"],
  "20230626": ["2", "1"],
  "20230627": ["2", "1"],
  "20230628": ["2", "1"],
  "20230629": ["2", "1"],
  "20230630": ["2", "1"]
};
const diretorio_agentes = {
  "Alcochete": [["Diagonalpotencial Lda", "Freeport Fashion Outlet, Avenida Euro 2004, Loja E32 B", "2890-154", "Alcochete"], ["Casa Ventura", "PraÃ§a JosÃ© Coelho nÂº47", "2890-212", "Samouco"]],
  "Almada": [["Flash PC*", "Largo Francisco Sanches, 8 Loja 14  Centro Comercial Santo Amaro", "2810-225", "Almada"], ["LADYXIC ", "PraÃ§a Lopes GraÃ§a, nÂº9, B", "2810-250", "Almada"], ["Papelaria Teorema", "Rua D. Manuel I 20B", "2810-259", "Almada"], ["Presselinha - Almada", "Almada Forum Lj. 181 Rua SÃ©rgio Malpique 2", "2810-354", "Almada"], ["Tech Ways*", "Rua CapitÃ£o LeitÃ£o 37A", "2800-135", "Almada"], ["Cantinho Doce", "Rua Alfredo Cunha 28, Loja 9", "2825-052", "Caparica"], ["Praia Vaz", "Rua SÃ£o LourenÃ§o Poente", "2825-023", "Caparica"], ["Papelaria Bela Vista", "Rua Cidade de Almada 1A", "2820-454", "Charneca Da Caparica"], ["Papyrus", "Avenida Elias Garcias n 908 B", "2820-222", "Charneca Da Caparica"], ["Decadas das Letras 2 - 62359", "Rua JosÃ© Viana 4 B", "2820-675", "Charneca De Caparica"], ["Electro Pescador Lda.", "PraÃ§a da Liberdade 17A Lj 11", "2825-322", "Costa De Caparica"], ["Papelaria GrÃ£o D'Areia", "Praceta JosÃ© Maria da Costa, 6 C", "2825-472", "Costa De Caparica"], ["Papelaria Jardim da FalÃ©sia", "Avenida do Oceano. 21 B", "2825-483", "Costa De Caparica"], ["Papelaria Tabacaria Capa D'ouro", "Rua Miguel Torga 45", "2825-343", "Costa De Caparica"]],
  "Barreiro": [["Casa Nestor", "Rua Lopo Soares de Albergaria, 19 D - Quinta da Lomba", "2830-197", "Alto Do Santo AndrÃ©"], ["CafÃ© com Arte", "R. Calouste Gulkenkian n 109", "2830-046", "Alto Do Seixalinho"], ["Papelaria Jackpot", "Rua Dr. Manuel Pacheco Nobre, 109 A", "2830-080", "Alto Do Seixalinho"], ["BD Telecommunication E Gift shop", "Rua Miguel Bombarda 207 A Barreiro", "2830-089", "Barreiro"], ["CafÃ© da Sorte", "Largo da Quinta Grande n.19C", "2830-249", "Barreiro"], ["Dona ChÃ¡vena - Barreiro", "Rua EÃ§a de QueirÃ³s N 20a  loja 6", "2830-344", "Barreiro"], ["Loja RubisgÃ¡s", "Rua Dr. EusÃ©bio LeÃ£o, 22 C", "2830-343", "Barreiro"], ["Ranufone 3", "PraÃ§a SÃ£o Francisco Xavier, NÂº11", "2830-153", "Barreiro"], ["Tabacaria - Mirasol", "Rua CapitÃ£es de Abril, 37 A - RC Esq", "2830-188", "Barreiro"], ["Universo RH Barreiro", "AV. ALFREDO DA SILVA, 32", "2830-302", "Barreiro"], ["CafÃ© Rossio", "Rua D. Manuel I 8", "2830-416", "Coina"], ["ParÃ¢metros e Desafios III", "EstaÃ§Ã£o FerroviÃ¡ria Coina, Galeria comercial loja1", "2830-406", "Coina"], ["Casa Jomar", "Av. Joaquim JosÃ© Fernandes, 28 A R/c", "2835-374", "Lavradio"], ["Papelaria Etika", "Rua Dr. Egas Moniz 17 - A", "2835-433", "Lavradio"], ["Tabacarias Vitorius", "Centro comercial Pingo Doce, loja 46 ", "2835-807", "Lavradio"], ["Papelaria Brivi", "Av. Escola dos Fuzileiros Navais n4", "2830-148", "Santo Andre"], ["Rolojogo", "Rua Afonso de Albuquerque, 80A", "2830-174", "Santo Andre"], ["PM - MediaÃ§Ã£o ImobiliÃ¡ria", "Rua Afonso de Albuquerque, 52 A", "2830-176", "Santo AndrÃ©"], ["Tabacaria TotoBonfim", "Rua Miguel Bombarda, 303", "2830-090", "Verderena"]],
  "Moita": [["Top line 2", "Largo Julio Dinis n1 R/chÃ£o ", "2860-199", "Alhos Vedros"], ["AgÃªncia Isabel Silva", "Rua Primeiro de Maio, NÂº37", "2835-147", "Baixa Da Banheira"], ["Hk TelecomicaÃ§Ãµes  e ReparaÃ§Ã£oes ", "Estrada Nacional n 11   nÂº 15  Centro Comercial Atlantis    loja 1   ", "2835-172", "Baixa Da Banheira"], ["Papelaria Gi", "Estrada Nacional 11-1", "2835-172", "Baixa Da Banheira"], ["Papelaria Gi - AML", "Estrada Nacional 11", "2835-172", "Baixa Da Banheira"], ["Ranufone 2", "Rua 1Âº de Maio, nÂº 102A", "2835-163", "Baixa Da Banheira"], ["SHAR ELECTRONICS AND TELEMOBILES", "RUA SAMORA MACHEL 37, R/C", "2835-158", "Baixa Da Banheira"], ["Papelaria Vimarto", "Rua Dr Alexandre Sequeira nÂº 14 A", "2860-458", "Moita"], ["Topline 1", "Praceta da Liberdade, lote 17, r/c dto", "2860-427", "Moita"], ["Topline 3", "Rua EÃ§a de Queiroz, nÂº6B", "2860-463", "Moita"], ["Topline Q1", "Largo Conde Ferreira, Quiosque", "2860-409", "Moita"], ["Ranufone", "Avenida Alfredo Dinis, 70 Loja 31", "2835-202", "Vale Da Amoreira"], ["Sahi TelecomunicaÃ§Ãµes", "Rua JosÃ© da ConceiÃ§Ã£o NÂº (C comercial) Lj 46", "2835-255", "Vale Da Amoreira"], ["Topline Q2", "Largo dos Cravos - Quiosque", "2835-208", "Vale Da Amoreira"]],
  "Montijo": [["Papelaria Machado", "Rua JosÃ© Quendera Miranda, 31 ", "2870-684", "Jardia"], ["Papelaria Salvador Web", "AV.LUIS DE CAMOES,29 ", "2870-287", "Montijo"], ["Presselinha - Alegro Montijo", "Alegro Montijo loja 019A", "2870-100", "Montijo"]],
  "Palmela": [["Minimercado Gomos Saber", "Bairro Padre Nabeto - R. de Angola, NÂº 67", "2950-115", "Palmela "], ["Pap Beautiful Things 2", "Rua Hermenegildo Capelo N80  ", "2950-234", "Palmela "], ["Pap.Beautiful Things 1", "AV DA LIBERDADE LOTE 8 LOJA B", "2950-201", "Palmela "], ["CafÃ© Mini-Mercado Sovipinhal", "Rua Febo Moniz, 12 Loja C", "2955-183", "Pinhal Novo"], ["Mini Mercado CaÃ§oete", "Rua Padre JosÃ© Estevens Dias, 133", "2955-212", "Pinhal Novo"]],
  "Seixal": [["Com Passo de Espera", "Gal. Com. EstaÃ§Ã£o FerroviÃ¡ria Fogueteiro, Loja 5", "2840-068", "Aldeia De Paio Pires"], ["Casjos Movel", "Rua 25 de Abril, 49 A", "2845-137", "Amora"], ["Domus Baia - PrestaÃ§Ã£o de ServiÃ§os", "Praceta 25 de Abril, nÂº16 e nÂº16A", "2845-040", "Amora"], ["FSA Telemoveis", "R DO MINHO, 20 RC", "2845-592", "Amora"], ["MISTIK FASHION BOUTIQUE", "RUA BAFATÃ�, 11 LOJA 27", "2845-050", "Amora"], ["Papelaria Bruno's", "Rua Movimento das forÃ§as Armadas 35B", "2845-380", "Amora"], ["Papelaria Edisa", "Rua 1Âº de Maio, 70 A, R/C Direito e Esquerdo", "2845-125", "Amora"], ["Shams Mobile", "Rua QtÂª da Medideira, 23 A", "2845-466", "Amora"], ["ZoobotÃ¢nica", "R. Infante Dom Augusto 36 A - Cruz de Pau", "2845-115", "Amora"], ["JR Papelaria", "Rua da casa do povo NÂº43 A - Loja 3", "2855-111", "Corroios"], ["Papelaria Catimar", "Rua de Niza, n11 Centro comercial Pierrot lj. 17 MilhaÃ§os", "2855-428", "Corroios"], ["Intermarche FernÃ£o Ferro ", "EdifÃ­cio IntermarchÃ©, R. da Casa Branca", "2865-015", "FernÃ£o Ferro"], ["Happyness 4 Pets", "R. LuÃ­s de CamÃµes lote 50 (ao lado do nr 24), Arrentela", "2840-440", "Seixal"], ["Papelaria Patygoo", "Rua da Paz 4502-A, Pinhal de Frades", "2840-316", "Seixal"]],
  "Sesimbra": [["Papelaria Nice", "Rua Gago Coutinho, Lote 2578", "2975-375", "Quinta Do Conde"], ["Planeta Rebelde", "Rua Abade Correia da Serra Loja 1B, Cotovia", "2970-382", "Sesimbra"]],
  "SetÃºbal": [["ArrÃ¡bida CafÃ©", "Rua 25 de Abril, 20A, Loja Esq.", "2925-459", "AzeitÃ£o"], ["Nuno Jorge Santos", "Rua JosÃ© Afonso 3 Loja B2925-073 AzeitÃ£o", "2925-073", "AzeitÃ£o"], ["Alertafuturo", "Av. D. JoÃ£o II, 6", "2910-548", "SetÃºbal"], ["CafÃ© \" A Ponte\" ", "R. Martim Afonso de Sousa n 11 ", "2910-419", "SetÃºbal"], ["Frutaria do Largo ", "Praca da Reboreda nÂº1 ", "2900-031", "SetÃºbal"], ["Jc InformÃ¡tica", "Pct. da Soc. Arqueol. LusitÃ¢nia, L13 - Lj 3", "2910-675", "SetÃºbal"], ["Mini-Mercado Escolha Certa", "Rua S. JoÃ£o de Deus, N146 Loja A", "2900-059", "SetÃºbal"], ["Papelaria Digarpa", "Av. de Angola, 16 A ", "2900-053", "SetÃºbal"], ["Papelaria Fonte Nova", "Rua Vasco da Gama, 55", "2900-180", "SetÃºbal"], ["Quiosque Palmeirinha", "Alameda das Palmeiras, Quiosque letra B", "2910-570", "SetÃºbal"], ["Sadosorte", "Avenida Jaime CortesÃ£o, 66", "2910-538", "SetÃºbal"], ["Super Centro Anaisa", "Rua Do Bairro Afonso Costa, NÂº 1 A 5", "2910- 414", "SetÃºbal"], ["set-link", "Av. Combatentes da Grande Guerra, 46-B, SetÃºbal", "2900-328", "SetÃºbal"]]
};
const diretorio_espacos_navegante = {
  "Alcochete": [["Rua Ruy Sousa Vinagre, EdifÃ­cio Monte Novo", "Seg-Sex 08:00-19:00"]],
  "Almada": [["Largo Alfredo Dinis, Cacilhas, 2800-252 Almada", "Seg-Sex 08:00-19:00"], ["Rua ErcÃ­lia Costa, nÂº 6, 2825-322 Costa da Caparica", "Seg-Dom 08:00-21:00"]],
  "Moita": [["PraÃ§a da RepÃºblica, 24", "Seg-Sex 08:00-19:00"]],
  "Montijo": [["PraÃ§a Gomes Freire de Andrade, 18", "Seg-Sex 08:00-19:00"]],
  "Palmela": [["EstaÃ§Ã£o RodoviÃ¡ria, Avenida 25 de Abril", "Seg-Sex 08:00-19:00"]],
  "Seixal": [["Avenida 25 de Abril, nÂº 9A R/C Esq., 2855-099 Corroios", "Seg-Sex 08:00-19:00"], ["Rua Joaquim Valentim Correia, nÂº 7, Cruz de Pau, 2845-568 Amora - Seixal", "Seg-Sex 08:00-19:00"]],
  "Sesimbra": [["Terminal RodoviÃ¡rio de Sesimbra, Avenida da Liberdade, 2970-635 Sesimbra", "Seg-Sex 08:00-19:00"]],
  "SetÃºbal": [["Interface de Transportes, PraÃ§a do Brasil", "Seg-Dom 08:00-21:00"], ["Vila Nogueira de AzeitÃ£o, Rua de Lisboa, 281", "Seg-Sex 08:00-19:00"]]
};
const diretorio_linhas = {
  "4001": ["Alcochete | Circular", "Pr\u00f3xima"],
  "4002": ["S\u00e3o Francisco | Circular", "Longa"],
  "4501": ["Alcochete - Montijo (Terminal Fluvial)", "Longa"],
  "4502": ["Alcochete - Passil", "Longa"],
  "4503": ["Atalaia - Jardia", "Longa"],
  "4504": ["Montijo (Terminal Fluvial) - Passil", "Longa"],
  "4510": ["Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio)", "Longa"],
  "4511": ["Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio), via Samouco", "Longa"],
  "4512": ["Alcochete (Freeport) - Set\u00fabal (ITS), via Alto Estanqueiro", "Longa"],
  "4513": ["Alcochete (Freeport) - Pinhal Novo", "Longa"],
  "4514": ["Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es", "Longa"],
  "4515": ["Montijo (Terminal Rodovi\u00e1rio) - Peg\u00f5es", "Longa"],
  "4516": ["Montijo (Terminal Rodovi\u00e1rio) - Rio Frio", "Longa"],
  "4600": ["Alcochete (Freeport) - Barreiro (Terminal)", "Longa"],
  "4702": ["Lisboa (Oriente) - Valbom", "R\u00e1pida"],
  "4703": ["Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio), via Alcochete e Samouco", "R\u00e1pida"],
  "4704": ["Atalaia - Lisboa (Oriente)", "R\u00e1pida"],
  "4705": ["Lisboa (Oriente) - Samouco", "R\u00e1pida"],
  "4706": ["S\u00e3o Francisco - Lisboa (Oriente)", "R\u00e1pida"],
  "3001": ["Almada (Cristo Rei) - Cacilhas (Terminal)", "Pr\u00f3xima"],
  "3002": ["Brevemente | Almada (Parque Urbano) - Pragal (Esta\u00e7\u00e3o)", "Longa"],
  "3003": ["Almada Forum - Cacilhas (Terminal)", "Longa"],
  "3004": ["Almada Forum - Marisol", "Longa"],
  "3005": ["Brevemente | Flexibus Almada | Circular", "Pr\u00f3xima"],
  "3006": ["Brevemente | Aroeira | Circular", "Pr\u00f3xima"],
  "3007": ["Bairro Fundo Fomento - Cacilhas (Terminal)", "Longa"],
  "3008": ["Ban\u00e1tica - Quintinha", "Longa"],
  "3009": ["Cacilhas (terminal - Trafaria (Terminal)", "Longa"],
  "3010": ["Cacilhas (Terminal) - Charneca da Caparica", "Longa"],
  "3011": ["Cacilhas (Terminal) - Costa da Caparica", "Longa"],
  "3012": ["Cacilhas (Terminal) - Fonte da Telha", "Longa"],
  "3013": ["Cacilhas (Terminal) - Monte da Caparica", "Longa"],
  "3014": ["Cacilhas (terminal) - Raposeira", "Longa"],
  "3015": ["Charneca da Caparica - Cova do Vapor", "Longa"],
  "3016": ["Brevemente | Charneca da Caparica - Lazarim", "Longa"],
  "3017": ["Charneca da Caparica - Pragal (Esta\u00e7\u00e3o)", "Longa"],
  "3018": ["Brevemente | Charneca da Caparica - Sobreda", "Longa"],
  "3019": ["Charneca da Caparica - Trafaria (Terminal)", "Longa"],
  "3020": ["Brevemente | Charneca da Caparica | Circular", "Pr\u00f3xima"],
  "3021": ["Costa da Caparica - Monte da Caparica (FCT)", "Longa"],
  "3022": ["Costa da Caparica (Terminal) - Hospital Garcia de Orta", "Longa"],
  "3023": ["Brevemente | Costa da Caparica (terminal) - Laranjeiro", "Longa"],
  "3024": ["Costa da Caparica (Terminal) - Pragal (Esta\u00e7\u00e3o)", "Longa"],
  "3025": ["Brevemente | Costa da Caparica (Terminal) - Pragal (Esta\u00e7\u00e3o), via IC20", "Longa"],
  "3026": ["Cova da Piedade - Hospital Garcia de Orta", "Pr\u00f3xima"],
  "3027": ["Hospital Garcia de Orta - Sobreda", "Longa"],
  "3028": ["Brevemente | Lazarim | Circular", "Pr\u00f3xima"],
  "3029": ["Brevemente | Marco Caba\u00e7o | Circular", "Pr\u00f3xima"],
  "3030": ["Fonte da Telha - Monte da Caparica (FCT)", "Longa"],
  "3031": ["Brevemente | Monte da Caparica - Quintinha", "Longa"],
  "3032": ["Brevemente | Monte da Caparica (FCT) - Quinta do Texugo", "Longa"],
  "3033": ["Brevemente | Monte da Caparica | Circular", "Pr\u00f3xima"],
  "3034": ["Porto Brand\u00e3o (Terminal) - Quinta do Texugo", "Longa"],
  "3035": ["Pragal (Esta\u00e7\u00e3o) - Quinta do Texugo", "Longa"],
  "3036": ["Pragal (Esta\u00e7\u00e3o) - Vale Flores", "Longa"],
  "3037": ["Brevemente | Quintinha | Circular", "Pr\u00f3xima"],
  "3501": ["Brevemente | Almada Forum - Marisol, via Sobreda", "Longa"],
  "3502": ["Almada Forum - Paio Pires (Centro)", "Longa"],
  "3503": ["Brevemente | Almada Forum - Vale de Milha\u00e7os", "Longa"],
  "3504": ["Brevemente | Bairro Fundo Fomento - Quintinha", "Longa"],
  "3505": ["Brevemente | Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o)", "Longa"],
  "3506": ["Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o), via Miratejo", "Longa"],
  "3507": ["Cacilhas (Terminal) - Marisol", "Longa"],
  "3508": ["Cacilhas (Terminal) - Paio Pires (Centro)", "Longa"],
  "3509": ["Cacilhas (Terminal) - Paio Pires (Centro), via Seixal (Terminal Fluvial) e Amora", "Longa"],
  "3510": ["Cacilhas (Terminal) - Pilotos", "Longa"],
  "3511": ["Cacilhas (Terminal) - Pinheirinho", "Longa"],
  "3512": ["Cacilhas (Terminal) - Quinta Princesa", "Longa"],
  "3513": ["Cacilhas (Terminal) - Santa Marta do Pinhal", "Longa"],
  "3514": ["Cacilhas (Terminal) - Vale de Milha\u00e7os", "Longa"],
  "3515": ["Caparica (Pilotos) -  Corroios", "Longa"],
  "3516": ["Charneca da Caparica - Corroios (Esta\u00e7\u00e3o)", "Longa"],
  "3517": ["Chegadinho - Corroios (Esta\u00e7\u00e3o)", "Longa"],
  "3518": ["Corroios (Esta\u00e7\u00e3o) - Vale de Figueira", "Longa"],
  "3519": ["Costa da Caparica (Terminal) - Corroios (Esta\u00e7\u00e3o)", "Longa"],
  "3520": ["Costa da Caparica (Terminal) - Quinta do Brasileiro", "Longa"],
  "3521": ["Cruz de Pau - Fonta da Telha", "Longa"],
  "3522": ["Fonte da Telha - Paio Pires (Centro)", "Longa"],
  "3523": ["Fonte da Telha - Paio Pires (Quinta Flam\u00e2ncia), via Seixal (Terminal Fluvial) e Foros de Amora (Esta\u00e7\u00e3o)", "Longa"],
  "3524": ["Hospital Garcia de Orta - Marisol", "Longa"],
  "3525": ["Hospital Garcia de Orta - Miratejo", "Longa"],
  "3526": ["Laranjeiro - Pinheirinho", "Longa"],
  "3527": ["Monte da Caparica (FCT) - Paio Pires (Bairro Cucena)", "Longa"],
  "3528": ["Monte da Caparica (FCT) - Paio Pires (Centro)", "Longa"],
  "3535": ["Cacilhas (Terminal) - Quinta do Conde", "Longa"],
  "3536": ["Cacilhas (Terminal) - Sesimbra (Terminal)", "Longa"],
  "3601": ["Barreiro - Cova da Piedade (Metro)", "Longa"],
  "3605": ["Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o", "Longa"],
  "3610": ["Brevemente | Cacilhas (Terminal) - Set\u00fabal (ITS), via A2", "Longa"],
  "3701": ["Brevemente | Almada (Centro Sul) - Alg\u00e9s (Terminal)", "Longa"],
  "3702": ["Almada (Parque Urbano) - Lisboa (C. Universit\u00e1ria)", "Longa"],
  "3703": ["Almada (Parque Urbano) - Lisboa (Sete Rios)", "Longa"],
  "3704": ["Charneca da Caparica - Lisboa (M. Pombal)", "Longa"],
  "3705": ["Brevemente | Charneca da Caparica - Lisboa (Sete Rios)", "Longa"],
  "3706": ["Brevemente | Charneca da Caparica - Lisboa (Sete Rios), via Feij\u00f3", "Longa"],
  "3707": ["Charneca da Caparica - Lisboa (Sete Rios), via Sobreda", "Longa"],
  "3708": ["Brevemente | Costa da Caparica (Terminal) - Lisboa (C. Sodr\u00e9)", "Longa"],
  "3709": ["Costa da Caparica (Terminal) - Lisboa (M. Pombal)", "Longa"],
  "3710": ["Costa da Caparica (Terminal) - Lisboa (Sete Rios)", "Longa"],
  "3711": ["Monte da Caparica (FCT) - Lisboa (Sete Rios)", "Longa"],
  "3715": ["Lisboa (M. Pombal) - Santa Marta do Pinhal", "Longa"],
  "3716": ["Lisboa (Sete Rios) - Marisol", "Longa"],
  "3717": ["Lisboa (Sete Rios) - Quinta do Brasileiro", "Longa"],
  "3720": ["Lisboa (Sete Rios) - Quinta do Conde", "R\u00e1pida"],
  "3721": ["Lisboa (Sete Rios) - Sesimbra (Terminal)", "R\u00e1pida"],
  "4725": ["Lisboa (Sete Rios) - Set\u00fabal (ITS)", "R\u00e1pida"],
  "3601": ["Barreiro - Cova da Piedade (Metro)", "Longa"],
  "3605": ["Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o", "Longa"],
  "3615": ["Barreiro - Seixal", "Longa"],
  "3620": ["Coina (Esta\u00e7\u00e3o) - Quinta do Conde", "Longa"],
  "3625": ["Brevemente | Barreiro - Sesimbra (Terminal)", "Longa"],
  "3626": ["Coina (Esta\u00e7\u00e3o) - Vila Fresca de Azeit\u00e3o", "Longa"],
  "3650": ["Brevemente | Moita - Sesimbra (Terminal)", "Mar"],
  "4600": ["Alcochete (Freeport) - Barreiro (Terminal)", "Longa"],
  "4601": ["Barreiro (Terminal) - Montijo (Terminal Rodovi\u00e1rio)", "Longa"],
  "4602": ["Alhos Vedros (Esta\u00e7\u00e3o) - Barreiro (Terminal)", "Longa"],
  "4603": ["Barreiro (Terminal) - Ch\u00e3o Duro", "Longa"],
  "4604": ["Barreiro (Terminal) - Moita (Escola Fragata do Tejo)", "Longa"],
  "4605": ["Lavradio - Pinhal do Forno", "Longa"],
  "4610": ["Bairro dos Marinheiros - Barreiro (Terminal)", "Longa"],
  "4611": ["Penalva - Moita (Esc. Secund\u00e1ria)", "Longa"],
  "4612": ["Bairro dos Marinheiros - Palmela (Terminal)", "Longa"],
  "4620": ["Moita - Paio Pires", "Longa"],
  "4621": ["Moita - Seixal (Terminal Fluvial)", "Longa"],
  "4630": ["Corroios (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
  "4631": ["Fogueteiro (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
  "3701": ["Brevemente | Almada (Centro Sul) - Alg\u00e9s (Terminal)", "Longa"],
  "3702": ["Almada (Parque Urbano) - Lisboa (C. Universit\u00e1ria)", "Longa"],
  "3703": ["Almada (Parque Urbano) - Lisboa (Sete Rios)", "Longa"],
  "3704": ["Charneca da Caparica - Lisboa (M. Pombal)", "Longa"],
  "3705": ["Brevemente | Charneca da Caparica - Lisboa (Sete Rios)", "Longa"],
  "3706": ["Brevemente | Charneca da Caparica - Lisboa (Sete Rios), via Feij\u00f3", "Longa"],
  "3707": ["Charneca da Caparica - Lisboa (Sete Rios), via Sobreda", "Longa"],
  "3708": ["Brevemente | Costa da Caparica (Terminal) - Lisboa (C. Sodr\u00e9)", "Longa"],
  "3709": ["Costa da Caparica (Terminal) - Lisboa (M. Pombal)", "Longa"],
  "3710": ["Costa da Caparica (Terminal) - Lisboa (Sete Rios)", "Longa"],
  "3711": ["Monte da Caparica (FCT) - Lisboa (Sete Rios)", "Longa"],
  "3715": ["Lisboa (M. Pombal) - Santa Marta do Pinhal", "Longa"],
  "3716": ["Lisboa (Sete Rios) - Marisol", "Longa"],
  "3717": ["Lisboa (Sete Rios) - Quinta do Brasileiro", "Longa"],
  "3720": ["Lisboa (Sete Rios) - Quinta do Conde", "R\u00e1pida"],
  "3721": ["Lisboa (Sete Rios) - Sesimbra (Terminal)", "R\u00e1pida"],
  "4701": ["Lisboa (Oriente) - Vale da Amoreira", "R\u00e1pida"],
  "4702": ["Lisboa (Oriente) - Valbom", "R\u00e1pida"],
  "4703": ["Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio), via Alcochete e Samouco", "R\u00e1pida"],
  "4704": ["Atalaia - Lisboa (Oriente)", "R\u00e1pida"],
  "4705": ["Lisboa (Oriente) - Samouco", "R\u00e1pida"],
  "4706": ["S\u00e3o Francisco - Lisboa (Oriente)", "R\u00e1pida"],
  "4707": ["Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio)", "R\u00e1pida"],
  "4710": ["Lisboa (Oriente) - Palmela (Terminal)", "R\u00e1pida"],
  "4711": ["Lisboa (Oriente) - Pinhal Novo", "R\u00e1pida"],
  "4715": ["Lisboa (Oriente) - Set\u00fabal (ITS), via Pinhal Novo", "R\u00e1pida"],
  "4720": ["Lisboa (Oriente) - Set\u00fabal (ITS)", "R\u00e1pida"],
  "4725": ["Lisboa (Sete Rios) - Set\u00fabal (ITS)", "R\u00e1pida"],
  "3650": ["Brevemente | Moita - Sesimbra (Terminal)", "Mar"],
  "4101": ["Alhos Vedros (Escola Jos\u00e9 Afonso) - Arroteias", "Longa"],
  "4102": ["Cabe\u00e7o Verde - Sarilhos Pequenos", "Longa"],
  "4103": ["Moita (Esta\u00e7\u00e3o) - Sarilhos Pequenos", "Pr\u00f3xima"],
  "4104": ["Moita | Circular", "Pr\u00f3xima"],
  "4530": ["Bairro Vila Morena - Pinhal Novo", "Longa"],
  "4531": ["Moita - Palmela (Terminal)", "Longa"],
  "4532": ["Moita - Quatro Marcos", "Longa"],
  "4600": ["Alcochete (Freeport) - Barreiro (Terminal)", "Longa"],
  "4601": ["Barreiro (Terminal) - Montijo (Terminal Rodovi\u00e1rio)", "Longa"],
  "4602": ["Alhos Vedros (Esta\u00e7\u00e3o) - Barreiro (Terminal)", "Longa"],
  "4603": ["Barreiro (Terminal) - Ch\u00e3o Duro", "Longa"],
  "4604": ["Barreiro (Terminal) - Moita (Escola Fragata do Tejo)", "Longa"],
  "4605": ["Lavradio - Pinhal do Forno", "Longa"],
  "4610": ["Bairro dos Marinheiros - Barreiro (Terminal)", "Longa"],
  "4611": ["Penalva - Moita (Esc. Secund\u00e1ria)", "Longa"],
  "4620": ["Moita - Paio Pires", "Longa"],
  "4621": ["Moita - Seixal (Terminal Fluvial)", "Longa"],
  "4701": ["Lisboa (Oriente) - Vale da Amoreira", "R\u00e1pida"],
  "4201": ["Afonsoeiro - Bairro da Liberdade", "Longa"],
  "4202": ["Afonsoeiro - Bairro do Saldanha, via Bairro da Cal\u00e7ada", "Longa"],
  "4203": ["Afonsoeiro - Montijo (Terminal Fluvial), via Bairro da Liberdade", "Pr\u00f3xima"],
  "4204": ["Bairro do Charqueir\u00e3o - Montijo (Terminal Fluvial)", "Pr\u00f3xima"],
  "4205": ["Bairro do Charqueir\u00e3o - Montijo (Terminal Fluvial), via Vale Porim", "Longa"],
  "4206": ["Bairro Esteval - Montijo (Terminal Fluvial)", "Pr\u00f3xima"],
  "4207": ["Montijo (\u00c1rea Comercial) - Montijo (Terminal Fluvial)", "Pr\u00f3xima"],
  "4208": ["Montijo (Terminal Rodovi\u00e1rio) - Sarilhos Grandes (Estr. 4 Marcos)", "Longa"],
  "4210": ["Canha - Foros Boavista", "Longa"],
  "4211": ["Craveiras - Peg\u00f5es | Circular", "Pr\u00f3xima"],
  "4212": ["Foros Boavista - Peg\u00f5es", "Longa"],
  "4501": ["Alcochete - Montijo (Terminal Fluvial)", "Longa"],
  "4502": ["Alcochete - Passil", "Longa"],
  "4503": ["Atalaia - Jardia", "Longa"],
  "4504": ["Montijo (Terminal Fluvial) - Passil", "Longa"],
  "4510": ["Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio)", "Longa"],
  "4511": ["Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio), via Samouco", "Longa"],
  "4512": ["Alcochete (Freeport) - Set\u00fabal (ITS), via Alto Estanqueiro", "Longa"],
  "4513": ["Alcochete (Freeport) - Pinhal Novo", "Longa"],
  "4514": ["Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es", "Longa"],
  "4515": ["Montijo (Terminal Rodovi\u00e1rio) - Peg\u00f5es", "Longa"],
  "4516": ["Montijo (Terminal Rodovi\u00e1rio) - Rio Frio", "Longa"],
  "4517": ["Montijo (Terminal Rodovi\u00e1rio) - Set\u00fabal (ITS)", "Longa"],
  "4520": ["Faias - Peg\u00f5es", "Longa"],
  "4521": ["Faias - Pinhal Novo", "Longa"],
  "4522": ["Faias - Poceir\u00e3o", "Longa"],
  "4523": ["Montijo (Terminal Rodovi\u00e1rio) - Pinhal Novo", "Longa"],
  "4524": ["Palmela (Terminal) - Peg\u00f5es", "Longa"],
  "4530": ["Bairro Vila Morena - Pinhal Novo", "Longa"],
  "4600": ["Alcochete (Freeport) - Barreiro (Terminal)", "Longa"],
  "4601": ["Barreiro (Terminal) - Montijo (Terminal Rodovi\u00e1rio)", "Longa"],
  "4643": ["Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)", "Mar"],
  "4701": ["Lisboa (Oriente) - Vale da Amoreira", "R\u00e1pida"],
  "4703": ["Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio), via Alcochete e Samouco", "R\u00e1pida"],
  "4704": ["Atalaia - Lisboa (Oriente)", "R\u00e1pida"],
  "4705": ["Lisboa (Oriente) - Samouco", "R\u00e1pida"],
  "4706": ["S\u00e3o Francisco - Lisboa (Oriente)", "R\u00e1pida"],
  "4707": ["Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio)", "R\u00e1pida"],
  "4902": ["Landeira - Peg\u00f5es", "Longa"],
  "4905": ["Faias - Vendas Novas", "Longa/ Inter-regional"],
  "4906": ["Set\u00fabal (ITS) - Vendas Novas, via Landeira", "Longa/ Inter-regional"],
  "3630": ["Brevemente | Azeit\u00e3o - Penalva (Esta\u00e7\u00e3o)", "Longa"],
  "4301": ["Palmela (Centro) - Palmela (Terminal)", "Pr\u00f3xima"],
  "4302": ["Palmela (Esta\u00e7\u00e3o) - Palmela (Terminal)", "Longa"],
  "4303": ["Palmela | Circular", "Pr\u00f3xima"],
  "4304": ["Palmela (Terminal) - Penalva", "Longa"],
  "4305": ["Brejos do Assa - Palmela (Terminal)", "Longa"],
  "4306": ["Cabanas - Palmela (Terminal)", "Longa"],
  "4307": ["Loja Nova - Palmela (Terminal)", "Longa"],
  "4308": ["Palmela (Terminal) - Pinhal Novo (Esta\u00e7\u00e3o)", "Longa"],
  "4310": ["\u00c1guas de Moura - Poceir\u00e3o", "Longa"],
  "4311": ["Asseiceira - Poceir\u00e3o", "Longa"],
  "4312": ["Poceir\u00e3o - Vale Abrunheira (X), via Fernando P\u00f3", "Longa"],
  "4313": ["Cabanas - Penalva", "Longa"],
  "4320": ["Pinhal Novo | Circular", "Pr\u00f3xima"],
  "4321": ["Pinhal Novo - Qta do Anjo", "Longa"],
  "4322": ["Pinhal Novo - Rio Frio", "Longa"],
  "4512": ["Alcochete (Freeport) - Set\u00fabal (ITS), via Alto Estanqueiro", "Longa"],
  "4513": ["Alcochete (Freeport) - Pinhal Novo", "Longa"],
  "4514": ["Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es", "Longa"],
  "4515": ["Montijo (Terminal Rodovi\u00e1rio) - Peg\u00f5es", "Longa"],
  "4516": ["Montijo (Terminal Rodovi\u00e1rio) - Rio Frio", "Longa"],
  "4517": ["Montijo (Terminal Rodovi\u00e1rio) - Set\u00fabal (ITS)", "Longa"],
  "4520": ["Faias - Peg\u00f5es", "Longa"],
  "4521": ["Faias - Pinhal Novo", "Longa"],
  "4522": ["Faias - Poceir\u00e3o", "Longa"],
  "4523": ["Montijo (Terminal Rodovi\u00e1rio) - Pinhal Novo", "Longa"],
  "4524": ["Palmela (Terminal) - Peg\u00f5es", "Longa"],
  "4530": ["Bairro Vila Morena - Pinhal Novo", "Longa"],
  "4531": ["Moita - Palmela (Terminal)", "Longa"],
  "4532": ["Moita - Quatro Marcos", "Longa"],
  "4540": ["\u00c1guas de Moura - Set\u00fabal (ITS)", "Longa"],
  "4541": ["Algeruz - Set\u00fabal (Av. Lu\u00edsa Todi)", "Longa"],
  "4542": ["Algeruz - Set\u00fabal (ITS)", "Longa"],
  "4543": ["Algeruz - Set\u00fabal (ITS), via Po\u00e7oilos", "Longa"],
  "4544": ["Bairro Marga\u00e7a - Set\u00fabal (ITS)", "Longa"],
  "4545": ["Biscainho - Set\u00fabal (Bela Vista)", "Longa"],
  "4546": ["Biscainho - Set\u00fabal (ITS)", "Longa"],
  "4547": ["Cabanas - Set\u00fabal (ITS)", "Longa"],
  "4548": ["Lagame\u00e7as - Set\u00fabal (ITS)", "Longa"],
  "4549": ["Palmela (Terminal) - Set\u00fabal (ITS)", "Longa"],
  "4550": ["Palmela (Terminal) - Vila Nogueira de Azeit\u00e3o", "Longa"],
  "4551": ["Palmela (USF) - Set\u00fabal (Av. Lu\u00edsa Todi)", "Longa"],
  "4560": ["Cabanas - Vila Nogueira de Azeit\u00e3o", "Longa"],
  "4561": ["Cabanas - Vila Nogueira de Azeit\u00e3o, via Quinta do Pic\u00e3o", "Longa"],
  "4562": ["Set\u00fabal (ITS) - Vila Nogueira de Azeit\u00e3o, via Palmela (Esta\u00e7\u00e3o)", "Longa"],
  "4610": ["Bairro dos Marinheiros - Barreiro (Terminal)", "Longa"],
  "4611": ["Penalva - Moita (Esc. Secund\u00e1ria)", "Longa"],
  "4612": ["Bairro dos Marinheiros - Palmela (Terminal)", "Longa"],
  "4710": ["Lisboa (Oriente) - Palmela (Terminal)", "R\u00e1pida"],
  "4711": ["Lisboa (Oriente) - Pinhal Novo", "R\u00e1pida"],
  "4715": ["Lisboa (Oriente) - Set\u00fabal (ITS), via Pinhal Novo", "R\u00e1pida"],
  "4901": ["Landeira - Set\u00fabal (ITS)", "Longa"],
  "4902": ["Landeira - Peg\u00f5es", "Longa"],
  "4905": ["Faias - Vendas Novas", "Longa/ Inter-regional"],
  "4906": ["Set\u00fabal (ITS) - Vendas Novas, via Landeira", "Longa/ Inter-regional"],
  "3101": ["Amora - Foros de Amora (Esta\u00e7\u00e3o)", "Pr\u00f3xima"],
  "3102": ["Brevemente | Aroeira - Paio Pires (Quinta Flam\u00e2ncia)", "Longa"],
  "3103": ["Corroios (Esta\u00e7\u00e3o) - Paio Pires (Farinheiras)", "Longa"],
  "3104": ["Corroios (Esta\u00e7\u00e3o) - Vale de Milha\u00e7os", "Longa"],
  "3105": ["Fern\u00e3o Ferro - Fogueteiro (Esta\u00e7\u00e3o)", "Longa"],
  "3106": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Fern\u00e3o Ferro", "Pr\u00f3xima"],
  "3107": ["Laranjeiras - Marco do Grilo", "Pr\u00f3xima"],
  "3108": ["Fogueteiro - Foros de Amora (Esta\u00e7\u00e3o)", "Pr\u00f3xima"],
  "3109": ["Fogueteiro (Esta\u00e7\u00e3o) - Parque Empresarial do Seixal", "Longa"],
  "3110": ["Fogueteiro (Esta\u00e7\u00e3o) - Redondos", "Longa"],
  "3111": ["Fogueteiro (Esta\u00e7\u00e3o) - Seixal (Terminal Fluvial)", "Longa"],
  "3112": ["Fogueteiro (Esta\u00e7\u00e3o) - Seixal (Terminal Fluvial), via Paio Pires", "Pr\u00f3xima"],
  "3113": ["Fogueteiro (Esta\u00e7\u00e3o) - Seixal (Terminal Fluvial), via Quinta do Cabral", "Pr\u00f3xima"],
  "3114": ["Foros de Amora - Paio Pires (Quinta Flam\u00e2ncia)", "Longa"],
  "3115": ["Brevemente | Marisol - Foros de Amora (Esta\u00e7\u00e3o), via Corroios (Esta\u00e7\u00e3o)", "Longa"],
  "3116": ["Brevemente | Marisol - Seixal (Terminal Fluvial), via Corroios (Esta\u00e7\u00e3o)", "Longa"],
  "3117": ["Brevemente | Marisol (Valadares) - Foros de Amora (Esta\u00e7\u00e3o)", "Longa"],
  "3118": ["Brevemente | Marisol (Valadares) - Seixal (Terminal Fluvial)", "Longa"],
  "3119": ["Pinhal Conde Cunha - Seixal (Terminal Fluvial)", "Longa"],
  "3120": ["Redondos - Seixal (Terminal Fluvial)", "Longa"],
  "3121": ["Seixal | Circular", "Pr\u00f3xima"],
  "3122": ["Brevemente | Verdizela - Cruz de Pau", "Longa"],
  "3501": ["Brevemente | Almada Forum - Marisol, via Sobreda", "Longa"],
  "3502": ["Almada Forum - Paio Pires (Centro)", "Longa"],
  "3503": ["Brevemente | Almada Forum - Vale de Milha\u00e7os", "Longa"],
  "3504": ["Brevemente | Bairro Fundo Fomento - Quintinha", "Longa"],
  "3505": ["Brevemente | Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o)", "Longa"],
  "3506": ["Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o), via Miratejo", "Longa"],
  "3507": ["Cacilhas (Terminal) - Marisol", "Longa"],
  "3508": ["Cacilhas (Terminal) - Paio Pires (Centro)", "Longa"],
  "3509": ["Cacilhas (Terminal) - Paio Pires (Centro), via Seixal (Terminal Fluvial) e Amora", "Longa"],
  "3510": ["Cacilhas (Terminal) - Pilotos", "Longa"],
  "3511": ["Cacilhas (Terminal) - Pinheirinho", "Longa"],
  "3512": ["Cacilhas (Terminal) - Quinta Princesa", "Longa"],
  "3513": ["Cacilhas (Terminal) - Santa Marta do Pinhal", "Longa"],
  "3514": ["Cacilhas (Terminal) - Vale de Milha\u00e7os", "Longa"],
  "3515": ["Caparica (Pilotos) -  Corroios", "Longa"],
  "3516": ["Charneca da Caparica - Corroios (Esta\u00e7\u00e3o)", "Longa"],
  "3517": ["Chegadinho - Corroios (Esta\u00e7\u00e3o)", "Longa"],
  "3518": ["Corroios (Esta\u00e7\u00e3o) - Vale de Figueira", "Longa"],
  "3519": ["Costa da Caparica (Terminal) - Corroios (Esta\u00e7\u00e3o)", "Longa"],
  "3520": ["Costa da Caparica (Terminal) - Quinta do Brasileiro", "Longa"],
  "3521": ["Cruz de Pau - Fonta da Telha", "Longa"],
  "3522": ["Fonte da Telha - Paio Pires (Centro)", "Longa"],
  "3523": ["Fonte da Telha - Paio Pires (Quinta Flam\u00e2ncia), via Seixal (Terminal Fluvial) e Foros de Amora (Esta\u00e7\u00e3o)", "Longa"],
  "3524": ["Hospital Garcia de Orta - Marisol", "Longa"],
  "3525": ["Hospital Garcia de Orta - Miratejo", "Longa"],
  "3526": ["Laranjeiro - Pinheirinho", "Longa"],
  "3527": ["Monte da Caparica (FCT) - Paio Pires (Bairro Cucena)", "Longa"],
  "3528": ["Monte da Caparica (FCT) - Paio Pires (Centro)", "Longa"],
  "3535": ["Cacilhas (Terminal) - Quinta do Conde", "Longa"],
  "3536": ["Cacilhas (Terminal) - Sesimbra (Terminal)", "Longa"],
  "3540": ["Brevemente | Alfarim - Coina (Esta\u00e7\u00e3o)", "Longa"],
  "3541": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Fern\u00e3o Ferro, via Casal do Sapo e Pinhal do General", "Longa"],
  "3542": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Praia do Meco", "Longa"],
  "3543": ["Coina (Esta\u00e7\u00e3o) - Quinta do Conde, via Estrada de Coina", "Longa"],
  "3544": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal)", "Longa"],
  "3545": ["Corroios (Esta\u00e7\u00e3o) - Sesimbra (Terminal)", "Longa"],
  "3546": ["Cruz de Pau - Quinta do Conde", "Longa"],
  "3547": ["Fogueteiro (Esta\u00e7\u00e3o) - Quinta do Conde", "Longa"],
  "3548": ["Marco do Grilo - Quinta do Conde", "Longa"],
  "3549": ["Quinta do Conde - Sesimbra (Terminal), via Sampaio e Marco do Grilo", "Longa"],
  "3601": ["Barreiro - Cova da Piedade (Metro)", "Longa"],
  "3605": ["Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o", "Longa"],
  "3610": ["Brevemente | Cacilhas (Terminal) - Set\u00fabal (ITS), via A2", "Longa"],
  "3615": ["Barreiro - Seixal", "Longa"],
  "3620": ["Coina (Esta\u00e7\u00e3o) - Quinta do Conde", "Longa"],
  "3625": ["Brevemente | Barreiro - Sesimbra (Terminal)", "Longa"],
  "3626": ["Coina (Esta\u00e7\u00e3o) - Vila Fresca de Azeit\u00e3o", "Longa"],
  "3635": ["Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal), via Azeit\u00e3o e Sampaio", "Longa"],
  "3715": ["Lisboa (M. Pombal) - Santa Marta do Pinhal", "Longa"],
  "3716": ["Lisboa (Sete Rios) - Marisol", "Longa"],
  "3717": ["Lisboa (Sete Rios) - Quinta do Brasileiro", "Longa"],
  "3720": ["Lisboa (Sete Rios) - Quinta do Conde", "R\u00e1pida"],
  "3721": ["Lisboa (Sete Rios) - Sesimbra (Terminal)", "R\u00e1pida"],
  "4620": ["Moita - Paio Pires", "Longa"],
  "4621": ["Moita - Seixal (Terminal Fluvial)", "Longa"],
  "4630": ["Corroios (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
  "4631": ["Fogueteiro (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
  "3201": ["Aldeia do Meco - Sesimbra (Terminal), via Aiana", "Longa"],
  "3202": ["Alfarim - Sesimbra (Terminal), via Aiana", "Longa"],
  "3203": ["Azoia - Sesimbra (Terminal)", "Longa"],
  "3204": ["Azoia - Sesimbra (Terminal), via Serra da Azoia", "Longa"],
  "3205": ["Cabo Espichel - Sesimbra (Terminal)", "Longa"],
  "3206": ["Carrasqueira - Sesimbra (Terminal)", "Longa"],
  "3207": ["Carrasqueira - Sesimbra (Terminal), via Valbom e Sampaio", "Longa"],
  "3208": ["Fetais - Sesimbra (Terminal)", "Longa"],
  "3209": ["Fornos - Sesimbra (Terminal), via Aiana", "Longa"],
  "3210": ["Lagoa de Albufeira - Sesimbra (Terminal)", "Longa"],
  "3211": ["Lagoa de Albufeira - Sesimbra (Terminal), via Sampaio", "Longa"],
  "3212": ["Ma\u00e7\u00e3 (Rua Macieira) - Sesimbra (Terminal)", "Longa"],
  "3213": ["Pinhal de Cima - Sesimbra (Terminal) | Circular", "Longa"],
  "3214": ["Sampaio - Santana", "Longa"],
  "3215": ["Fornos - Sampaio", "Longa"],
  "3216": ["Alto das Vinhas - Sampaio", "Longa"],
  "3217": ["Azoia - Sampaio", "Longa"],
  "3218": ["Sesimbra (Porto de Abrigo) - Sesimbra (Terminal)", "Pr\u00f3xima"],
  "3219": ["Brevemente | Sesimbra (R. Palames) - Sesimbra (Terminal)", "Pr\u00f3xima"],
  "3220": ["Sesimbra | Circular", "Pr\u00f3xima"],
  "3221": ["Valbom - Sesimbra (Terminal), via Sampaio", "Longa"],
  "3222": ["Quinta do Conde | Circular", "Pr\u00f3xima"],
  "3535": ["Cacilhas (Terminal) - Quinta do Conde", "Longa"],
  "3536": ["Cacilhas (Terminal) - Sesimbra (Terminal)", "Longa"],
  "3540": ["Brevemente | Alfarim - Coina (Esta\u00e7\u00e3o)", "Longa"],
  "3541": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Fern\u00e3o Ferro, via Casal do Sapo e Pinhal do General", "Longa"],
  "3542": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Praia do Meco", "Longa"],
  "3543": ["Coina (Esta\u00e7\u00e3o) - Quinta do Conde, via Estrada de Coina", "Longa"],
  "3544": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal)", "Longa"],
  "3545": ["Corroios (Esta\u00e7\u00e3o) - Sesimbra (Terminal)", "Longa"],
  "3546": ["Cruz de Pau - Quinta do Conde", "Longa"],
  "3547": ["Fogueteiro (Esta\u00e7\u00e3o) - Quinta do Conde", "Longa"],
  "3548": ["Marco do Grilo - Quinta do Conde", "Longa"],
  "3549": ["Quinta do Conde - Sesimbra (Terminal), via Sampaio e Marco do Grilo", "Longa"],
  "3605": ["Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o", "Longa"],
  "3620": ["Coina (Esta\u00e7\u00e3o) - Quinta do Conde", "Longa"],
  "3625": ["Brevemente | Barreiro - Sesimbra (Terminal)", "Longa"],
  "3626": ["Coina (Esta\u00e7\u00e3o) - Vila Fresca de Azeit\u00e3o", "Longa"],
  "3630": ["Brevemente | Azeit\u00e3o - Penalva (Esta\u00e7\u00e3o)", "Longa"],
  "3635": ["Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal), via Azeit\u00e3o e Sampaio", "Longa"],
  "3640": ["Azoia - Vila Nogueira de Azeit\u00e3o", "Longa"],
  "3641": ["Quinta do Conde - Sampaio, via Azeit\u00e3o", "Longa"],
  "3650": ["Brevemente | Moita - Sesimbra (Terminal)", "Mar"],
  "3720": ["Lisboa (Sete Rios) - Quinta do Conde", "R\u00e1pida"],
  "3721": ["Lisboa (Sete Rios) - Sesimbra (Terminal)", "R\u00e1pida"],
  "4630": ["Corroios (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
  "4631": ["Fogueteiro (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
  "4640": ["Casais da Serra - Vila Nogueira de Azeit\u00e3o", "Longa"],
  "4641": ["Quinta do Conde - Set\u00fabal (ITS)", "Longa"],
  "4642": ["Sesimbra (Terminal) - Set\u00fabal (Hospital)", "Longa"],
  "4643": ["Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)", "Mar"],
  "3605": ["Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o", "Longa"],
  "3610": ["Brevemente | Cacilhas (Terminal) - Set\u00fabal (ITS), via A2", "Longa"],
  "3625": ["Brevemente | Barreiro - Sesimbra (Terminal)", "Longa"],
  "3626": ["Coina (Esta\u00e7\u00e3o) - Vila Fresca de Azeit\u00e3o", "Longa"],
  "3630": ["Brevemente | Azeit\u00e3o - Penalva (Esta\u00e7\u00e3o)", "Longa"],
  "3635": ["Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal), via Azeit\u00e3o e Sampaio", "Longa"],
  "3640": ["Azoia - Vila Nogueira de Azeit\u00e3o", "Longa"],
  "3641": ["Quinta do Conde - Sampaio, via Azeit\u00e3o", "Longa"],
  "4401": ["Cachofarra - Set\u00fabal (Hospital)", "Longa"],
  "4402": ["Estefanilha - Set\u00fabal (ITS)", "Longa"],
  "4403": ["Fonte da Talha - Set\u00fabal (Av. Lu\u00edsa Todi)", "Pr\u00f3xima"],
  "4404": ["Interfaces Set\u00fabal | Circular", "Pr\u00f3xima"],
  "4405": ["Livramento-Montebelo | Circular", "Longa"],
  "4406": ["Manteigadas - Set\u00fabal (Mercado)", "Pr\u00f3xima"],
  "4407": ["Manteigadas - Set\u00fabal (Mercado), via Bairro da Carmona", "Longa"],
  "4408": ["Manteigadas - Set\u00fabal (Mercado), via Bela Vista", "Longa"],
  "4409": ["Manteigadas - Viso", "Longa"],
  "4410": ["Manteigadas (Esc. Profissional) - Set\u00fabal (Alegro)", "Longa"],
  "4411": ["Morgada - Set\u00fabal (ITS)", "Longa"],
  "4412": ["Morgada - Set\u00fabal (Mercado)", "Longa"],
  "4413": ["Morgada - Set\u00fabal (Mercado), via Bela Vista", "Longa"],
  "4414": ["Out\u00e3o (Hospital) - Set\u00fabal (ITS)", "Longa"],
  "4415": ["Out\u00e3o (Hospital) - Set\u00fabal (ITS), via vale da Rasca", "Longa"],
  "4416": ["Po\u00e7o Mouro - Set\u00fabal (ITS)", "Longa"],
  "4417": ["Po\u00e7o Mouro - Set\u00fabal (ITS), via Manteigadas", "Longa"],
  "4418": ["Set\u00fabal (Alegro) - Set\u00fabal (Av. 5 Outubro)", "Longa"],
  "4419": ["Brejos Canes - Set\u00fabal (Saboaria)", "Longa"],
  "4420": ["Set\u00fabal (Alegro) - Set\u00fabal (ITS)", "Pr\u00f3xima"],
  "4421": ["Set\u00fabal (Bairro Camolas) - Set\u00fabal (Casal Figueiras)", "Longa"],
  "4422": ["Set\u00fabal (Bairro Camolas) - Set\u00fabal (Casal Figueiras), via Bairro do Viso", "Longa"],
  "4423": ["Amoreiras \u2013 Set\u00fabal (Av. Lu\u00edsa Todi)", "Pr\u00f3xima"],
  "4424": ["Set\u00fabal (Bairro Viso) - Manteigadas", "Longa"],
  "4425": ["Set\u00fabal (Escola Viso) - Mitrena", "Longa"],
  "4426": ["Set\u00fabal (Bairro Viso) - Set\u00fabal (CHEset\u00fabal)", "Pr\u00f3xima"],
  "4427": ["Set\u00fabal (Bela Vista) - Set\u00fabal (Mercado)", "Pr\u00f3xima"],
  "4428": ["Set\u00fabal (Casal Figueiras) - Vale Ana Gomes", "Longa"],
  "4429": ["Set\u00fabal (Centro Sa\u00fade) - Set\u00fabal (Mercado)", "Longa"],
  "4430": ["Set\u00fabal (Hospital) - Set\u00fabal (Montalv\u00e3o)", "Pr\u00f3xima"],
  "4431": ["Set\u00fabal (ITS) - Set\u00fabal (Quinta Varzinha)", "Pr\u00f3xima"],
  "4432": ["Set\u00fabal (ITS) - Vale de Choupo", "Longa"],
  "4433": ["Alto Guerra - Set\u00fabal (Casal Figueiras)", "Longa"],
  "4434": ["Set\u00fabal (Mercado 2 de Abril) - Set\u00fabal (R. Timor)", "Pr\u00f3xima"],
  "4435": ["Biscainho - Faralh\u00e3o", "Longa"],
  "4436": ["Set\u00fabal (Mercado) - Set\u00fabal (Av. Soeiro Pereira Gomes)", "Pr\u00f3xima"],
  "4437": ["Faralh\u00e3o - Set\u00fabal (ITS)", "Longa"],
  "4438": ["Set\u00fabal (Monte Belo Norte) - Set\u00fabal (Saboaria)", "Longa"],
  "4439": ["Praias do Sado - Set\u00fabal (ITS)", "Longa"],
  "4440": ["Set\u00fabal (Monte Belo Norte) - Set\u00fabal (Saboaria), via Alegro", "Longa"],
  "4441": ["Set\u00fabal (Saboaria) - Set\u00fabal (Vale Cobro)", "Longa"],
  "4442": ["Praias do Sado (Esta\u00e7\u00e3o) - Set\u00fabal (Bela Vista)", "Longa"],
  "4443": ["Set\u00fabal (Polit\u00e9cnico) - Praias do Sado", "Longa"],
  "4451": ["Mitrena (Lisnave) - Set\u00fabal (ITS)", "Longa"],
  "4452": ["Mitrena (Portucel) - Set\u00fabal (ITS)", "Longa"],
  "4453": ["Mitrena (Portucel) - Set\u00fabal (ITS), via Estrada Gra\u00e7a", "Longa"],
  "4460": ["Azeit\u00e3o | Circular", "Longa"],
  "4470": ["Brejos Azeit\u00e3o - Praia do Creiro", "Longa"],
  "4471": ["Praia Albarquel | Circular", "Longa"],
  "4472": ["Praia do Creiro - Set\u00fabal (ITS)", "Longa"],
  "4474": ["Figueirinha - Set\u00fabal (Alegro)", "Longa"],
  "4475": ["Portinho da Arr\u00e1bida - Viso", "Longa"],
  "4476": ["Praias Arr\u00e1bida | Circular", "Longa"],
  "4512": ["Alcochete (Freeport) - Set\u00fabal (ITS), via Alto Estanqueiro", "Longa"],
  "4517": ["Montijo (Terminal Rodovi\u00e1rio) - Set\u00fabal (ITS)", "Longa"],
  "4540": ["\u00c1guas de Moura - Set\u00fabal (ITS)", "Longa"],
  "4541": ["Algeruz - Set\u00fabal (Av. Lu\u00edsa Todi)", "Longa"],
  "4542": ["Algeruz - Set\u00fabal (ITS)", "Longa"],
  "4543": ["Algeruz - Set\u00fabal (ITS), via Po\u00e7oilos", "Longa"],
  "4544": ["Bairro Marga\u00e7a - Set\u00fabal (ITS)", "Longa"],
  "4545": ["Biscainho - Set\u00fabal (Bela Vista)", "Longa"],
  "4546": ["Biscainho - Set\u00fabal (ITS)", "Longa"],
  "4547": ["Cabanas - Set\u00fabal (ITS)", "Longa"],
  "4548": ["Lagame\u00e7as - Set\u00fabal (ITS)", "Longa"],
  "4549": ["Palmela (Terminal) - Set\u00fabal (ITS)", "Longa"],
  "4550": ["Palmela (Terminal) - Vila Nogueira de Azeit\u00e3o", "Longa"],
  "4551": ["Palmela (USF) - Set\u00fabal (Av. Lu\u00edsa Todi)", "Longa"],
  "4560": ["Cabanas - Vila Nogueira de Azeit\u00e3o", "Longa"],
  "4561": ["Cabanas - Vila Nogueira de Azeit\u00e3o, via Quinta do Pic\u00e3o", "Longa"],
  "4562": ["Set\u00fabal (ITS) - Vila Nogueira de Azeit\u00e3o, via Palmela (Esta\u00e7\u00e3o)", "Longa"],
  "4630": ["Corroios (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
  "4631": ["Fogueteiro (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
  "4640": ["Casais da Serra - Vila Nogueira de Azeit\u00e3o", "Longa"],
  "4641": ["Quinta do Conde - Set\u00fabal (ITS)", "Longa"],
  "4642": ["Sesimbra (Terminal) - Set\u00fabal (Hospital)", "Longa"],
  "4643": ["Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)", "Mar"],
  "4715": ["Lisboa (Oriente) - Set\u00fabal (ITS), via Pinhal Novo", "R\u00e1pida"],
  "4720": ["Lisboa (Oriente) - Set\u00fabal (ITS)", "R\u00e1pida"],
  "4725": ["Lisboa (Sete Rios) - Set\u00fabal (ITS)", "R\u00e1pida"],
  "4901": ["Landeira - Set\u00fabal (ITS)", "Longa"],
  "4906": ["Set\u00fabal (ITS) - Vendas Novas, via Landeira", "Longa/ Inter-regional"],
  "4901": ["Landeira - Set\u00fabal (ITS)", "Longa"],
  "4902": ["Landeira - Peg\u00f5es", "Longa"],
  "4905": ["Faias - Vendas Novas", "Longa/ Inter-regional"],
  "4906": ["Set\u00fabal (ITS) - Vendas Novas, via Landeira", "Longa/ Inter-regional"]
};
const diretorio_linhas_por_municipio = {
  "Alcochete": {
    "4001": ["Alcochete | Circular", "Pr\u00f3xima"],
    "4002": ["S\u00e3o Francisco | Circular", "Longa"],
    "4501": ["Alcochete - Montijo (Terminal Fluvial)", "Longa"],
    "4502": ["Alcochete - Passil", "Longa"],
    "4503": ["Atalaia - Jardia", "Longa"],
    "4504": ["Montijo (Terminal Fluvial) - Passil", "Longa"],
    "4510": ["Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio)", "Longa"],
    "4511": ["Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio), via Samouco", "Longa"],
    "4512": ["Alcochete (Freeport) - Set\u00fabal (ITS), via Alto Estanqueiro", "Longa"],
    "4513": ["Alcochete (Freeport) - Pinhal Novo", "Longa"],
    "4514": ["Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es", "Longa"],
    "4515": ["Montijo (Terminal Rodovi\u00e1rio) - Peg\u00f5es", "Longa"],
    "4516": ["Montijo (Terminal Rodovi\u00e1rio) - Rio Frio", "Longa"],
    "4600": ["Alcochete (Freeport) - Barreiro (Terminal)", "Longa"],
    "4702": ["Lisboa (Oriente) - Valbom", "R\u00e1pida"],
    "4703": ["Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio), via Alcochete e Samouco", "R\u00e1pida"],
    "4704": ["Atalaia - Lisboa (Oriente)", "R\u00e1pida"],
    "4705": ["Lisboa (Oriente) - Samouco", "R\u00e1pida"],
    "4706": ["S\u00e3o Francisco - Lisboa (Oriente)", "R\u00e1pida"]
  },
  "Almada": {
    "3001": ["Almada (Cristo Rei) - Cacilhas (Terminal)", "Pr\u00f3xima"],
    "3002": ["Brevemente | Almada (Parque Urbano) - Pragal (Esta\u00e7\u00e3o)", "Longa"],
    "3003": ["Almada Forum - Cacilhas (Terminal)", "Longa"],
    "3004": ["Almada Forum - Marisol", "Longa"],
    "3005": ["Brevemente | Flexibus Almada | Circular", "Pr\u00f3xima"],
    "3006": ["Brevemente | Aroeira | Circular", "Pr\u00f3xima"],
    "3007": ["Bairro Fundo Fomento - Cacilhas (Terminal)", "Longa"],
    "3008": ["Ban\u00e1tica - Quintinha", "Longa"],
    "3009": ["Cacilhas (terminal - Trafaria (Terminal)", "Longa"],
    "3010": ["Cacilhas (Terminal) - Charneca da Caparica", "Longa"],
    "3011": ["Cacilhas (Terminal) - Costa da Caparica", "Longa"],
    "3012": ["Cacilhas (Terminal) - Fonte da Telha", "Longa"],
    "3013": ["Cacilhas (Terminal) - Monte da Caparica", "Longa"],
    "3014": ["Cacilhas (terminal) - Raposeira", "Longa"],
    "3015": ["Charneca da Caparica - Cova do Vapor", "Longa"],
    "3016": ["Brevemente | Charneca da Caparica - Lazarim", "Longa"],
    "3017": ["Charneca da Caparica - Pragal (Esta\u00e7\u00e3o)", "Longa"],
    "3018": ["Brevemente | Charneca da Caparica - Sobreda", "Longa"],
    "3019": ["Charneca da Caparica - Trafaria (Terminal)", "Longa"],
    "3020": ["Brevemente | Charneca da Caparica | Circular", "Pr\u00f3xima"],
    "3021": ["Costa da Caparica - Monte da Caparica (FCT)", "Longa"],
    "3022": ["Costa da Caparica (Terminal) - Hospital Garcia de Orta", "Longa"],
    "3023": ["Brevemente | Costa da Caparica (terminal) - Laranjeiro", "Longa"],
    "3024": ["Costa da Caparica (Terminal) - Pragal (Esta\u00e7\u00e3o)", "Longa"],
    "3025": ["Brevemente | Costa da Caparica (Terminal) - Pragal (Esta\u00e7\u00e3o), via IC20", "Longa"],
    "3026": ["Cova da Piedade - Hospital Garcia de Orta", "Pr\u00f3xima"],
    "3027": ["Hospital Garcia de Orta - Sobreda", "Longa"],
    "3028": ["Brevemente | Lazarim | Circular", "Pr\u00f3xima"],
    "3029": ["Brevemente | Marco Caba\u00e7o | Circular", "Pr\u00f3xima"],
    "3030": ["Fonte da Telha - Monte da Caparica (FCT)", "Longa"],
    "3031": ["Brevemente | Monte da Caparica - Quintinha", "Longa"],
    "3032": ["Brevemente | Monte da Caparica (FCT) - Quinta do Texugo", "Longa"],
    "3033": ["Brevemente | Monte da Caparica | Circular", "Pr\u00f3xima"],
    "3034": ["Porto Brand\u00e3o (Terminal) - Quinta do Texugo", "Longa"],
    "3035": ["Pragal (Esta\u00e7\u00e3o) - Quinta do Texugo", "Longa"],
    "3036": ["Pragal (Esta\u00e7\u00e3o) - Vale Flores", "Longa"],
    "3037": ["Brevemente | Quintinha | Circular", "Pr\u00f3xima"],
    "3501": ["Brevemente | Almada Forum - Marisol, via Sobreda", "Longa"],
    "3502": ["Almada Forum - Paio Pires (Centro)", "Longa"],
    "3503": ["Brevemente | Almada Forum - Vale de Milha\u00e7os", "Longa"],
    "3504": ["Brevemente | Bairro Fundo Fomento - Quintinha", "Longa"],
    "3505": ["Brevemente | Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o)", "Longa"],
    "3506": ["Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o), via Miratejo", "Longa"],
    "3507": ["Cacilhas (Terminal) - Marisol", "Longa"],
    "3508": ["Cacilhas (Terminal) - Paio Pires (Centro)", "Longa"],
    "3509": ["Cacilhas (Terminal) - Paio Pires (Centro), via Seixal (Terminal Fluvial) e Amora", "Longa"],
    "3510": ["Cacilhas (Terminal) - Pilotos", "Longa"],
    "3511": ["Cacilhas (Terminal) - Pinheirinho", "Longa"],
    "3512": ["Cacilhas (Terminal) - Quinta Princesa", "Longa"],
    "3513": ["Cacilhas (Terminal) - Santa Marta do Pinhal", "Longa"],
    "3514": ["Cacilhas (Terminal) - Vale de Milha\u00e7os", "Longa"],
    "3515": ["Caparica (Pilotos) -  Corroios", "Longa"],
    "3516": ["Charneca da Caparica - Corroios (Esta\u00e7\u00e3o)", "Longa"],
    "3517": ["Chegadinho - Corroios (Esta\u00e7\u00e3o)", "Longa"],
    "3518": ["Corroios (Esta\u00e7\u00e3o) - Vale de Figueira", "Longa"],
    "3519": ["Costa da Caparica (Terminal) - Corroios (Esta\u00e7\u00e3o)", "Longa"],
    "3520": ["Costa da Caparica (Terminal) - Quinta do Brasileiro", "Longa"],
    "3521": ["Cruz de Pau - Fonta da Telha", "Longa"],
    "3522": ["Fonte da Telha - Paio Pires (Centro)", "Longa"],
    "3523": ["Fonte da Telha - Paio Pires (Quinta Flam\u00e2ncia), via Seixal (Terminal Fluvial) e Foros de Amora (Esta\u00e7\u00e3o)", "Longa"],
    "3524": ["Hospital Garcia de Orta - Marisol", "Longa"],
    "3525": ["Hospital Garcia de Orta - Miratejo", "Longa"],
    "3526": ["Laranjeiro - Pinheirinho", "Longa"],
    "3527": ["Monte da Caparica (FCT) - Paio Pires (Bairro Cucena)", "Longa"],
    "3528": ["Monte da Caparica (FCT) - Paio Pires (Centro)", "Longa"],
    "3535": ["Cacilhas (Terminal) - Quinta do Conde", "Longa"],
    "3536": ["Cacilhas (Terminal) - Sesimbra (Terminal)", "Longa"],
    "3601": ["Barreiro - Cova da Piedade (Metro)", "Longa"],
    "3605": ["Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o", "Longa"],
    "3610": ["Brevemente | Cacilhas (Terminal) - Set\u00fabal (ITS), via A2", "Longa"],
    "3701": ["Brevemente | Almada (Centro Sul) - Alg\u00e9s (Terminal)", "Longa"],
    "3702": ["Almada (Parque Urbano) - Lisboa (C. Universit\u00e1ria)", "Longa"],
    "3703": ["Almada (Parque Urbano) - Lisboa (Sete Rios)", "Longa"],
    "3704": ["Charneca da Caparica - Lisboa (M. Pombal)", "Longa"],
    "3705": ["Brevemente | Charneca da Caparica - Lisboa (Sete Rios)", "Longa"],
    "3706": ["Brevemente | Charneca da Caparica - Lisboa (Sete Rios), via Feij\u00f3", "Longa"],
    "3707": ["Charneca da Caparica - Lisboa (Sete Rios), via Sobreda", "Longa"],
    "3708": ["Brevemente | Costa da Caparica (Terminal) - Lisboa (C. Sodr\u00e9)", "Longa"],
    "3709": ["Costa da Caparica (Terminal) - Lisboa (M. Pombal)", "Longa"],
    "3710": ["Costa da Caparica (Terminal) - Lisboa (Sete Rios)", "Longa"],
    "3711": ["Monte da Caparica (FCT) - Lisboa (Sete Rios)", "Longa"],
    "3715": ["Lisboa (M. Pombal) - Santa Marta do Pinhal", "Longa"],
    "3716": ["Lisboa (Sete Rios) - Marisol", "Longa"],
    "3717": ["Lisboa (Sete Rios) - Quinta do Brasileiro", "Longa"],
    "3720": ["Lisboa (Sete Rios) - Quinta do Conde", "R\u00e1pida"],
    "3721": ["Lisboa (Sete Rios) - Sesimbra (Terminal)", "R\u00e1pida"],
    "4725": ["Lisboa (Sete Rios) - Set\u00fabal (ITS)", "R\u00e1pida"]
  },
  "Barreiro": {
    "3601": ["Barreiro - Cova da Piedade (Metro)", "Longa"],
    "3605": ["Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o", "Longa"],
    "3615": ["Barreiro - Seixal", "Longa"],
    "3620": ["Coina (Esta\u00e7\u00e3o) - Quinta do Conde", "Longa"],
    "3625": ["Brevemente | Barreiro - Sesimbra (Terminal)", "Longa"],
    "3626": ["Coina (Esta\u00e7\u00e3o) - Vila Fresca de Azeit\u00e3o", "Longa"],
    "3650": ["Brevemente | Moita - Sesimbra (Terminal)", "Mar"],
    "4600": ["Alcochete (Freeport) - Barreiro (Terminal)", "Longa"],
    "4601": ["Barreiro (Terminal) - Montijo (Terminal Rodovi\u00e1rio)", "Longa"],
    "4602": ["Alhos Vedros (Esta\u00e7\u00e3o) - Barreiro (Terminal)", "Longa"],
    "4603": ["Barreiro (Terminal) - Ch\u00e3o Duro", "Longa"],
    "4604": ["Barreiro (Terminal) - Moita (Escola Fragata do Tejo)", "Longa"],
    "4605": ["Lavradio - Pinhal do Forno", "Longa"],
    "4610": ["Bairro dos Marinheiros - Barreiro (Terminal)", "Longa"],
    "4611": ["Penalva - Moita (Esc. Secund\u00e1ria)", "Longa"],
    "4612": ["Bairro dos Marinheiros - Palmela (Terminal)", "Longa"],
    "4620": ["Moita - Paio Pires", "Longa"],
    "4621": ["Moita - Seixal (Terminal Fluvial)", "Longa"],
    "4630": ["Corroios (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
    "4631": ["Fogueteiro (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"]
  },
  "Lisboa": {
    "3701": ["Brevemente | Almada (Centro Sul) - Alg\u00e9s (Terminal)", "Longa"],
    "3702": ["Almada (Parque Urbano) - Lisboa (C. Universit\u00e1ria)", "Longa"],
    "3703": ["Almada (Parque Urbano) - Lisboa (Sete Rios)", "Longa"],
    "3704": ["Charneca da Caparica - Lisboa (M. Pombal)", "Longa"],
    "3705": ["Brevemente | Charneca da Caparica - Lisboa (Sete Rios)", "Longa"],
    "3706": ["Brevemente | Charneca da Caparica - Lisboa (Sete Rios), via Feij\u00f3", "Longa"],
    "3707": ["Charneca da Caparica - Lisboa (Sete Rios), via Sobreda", "Longa"],
    "3708": ["Brevemente | Costa da Caparica (Terminal) - Lisboa (C. Sodr\u00e9)", "Longa"],
    "3709": ["Costa da Caparica (Terminal) - Lisboa (M. Pombal)", "Longa"],
    "3710": ["Costa da Caparica (Terminal) - Lisboa (Sete Rios)", "Longa"],
    "3711": ["Monte da Caparica (FCT) - Lisboa (Sete Rios)", "Longa"],
    "3715": ["Lisboa (M. Pombal) - Santa Marta do Pinhal", "Longa"],
    "3716": ["Lisboa (Sete Rios) - Marisol", "Longa"],
    "3717": ["Lisboa (Sete Rios) - Quinta do Brasileiro", "Longa"],
    "3720": ["Lisboa (Sete Rios) - Quinta do Conde", "R\u00e1pida"],
    "3721": ["Lisboa (Sete Rios) - Sesimbra (Terminal)", "R\u00e1pida"],
    "4701": ["Lisboa (Oriente) - Vale da Amoreira", "R\u00e1pida"],
    "4702": ["Lisboa (Oriente) - Valbom", "R\u00e1pida"],
    "4703": ["Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio), via Alcochete e Samouco", "R\u00e1pida"],
    "4704": ["Atalaia - Lisboa (Oriente)", "R\u00e1pida"],
    "4705": ["Lisboa (Oriente) - Samouco", "R\u00e1pida"],
    "4706": ["S\u00e3o Francisco - Lisboa (Oriente)", "R\u00e1pida"],
    "4707": ["Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio)", "R\u00e1pida"],
    "4710": ["Lisboa (Oriente) - Palmela (Terminal)", "R\u00e1pida"],
    "4711": ["Lisboa (Oriente) - Pinhal Novo", "R\u00e1pida"],
    "4715": ["Lisboa (Oriente) - Set\u00fabal (ITS), via Pinhal Novo", "R\u00e1pida"],
    "4720": ["Lisboa (Oriente) - Set\u00fabal (ITS)", "R\u00e1pida"],
    "4725": ["Lisboa (Sete Rios) - Set\u00fabal (ITS)", "R\u00e1pida"]
  },
  "Moita": {
    "3650": ["Brevemente | Moita - Sesimbra (Terminal)", "Mar"],
    "4101": ["Alhos Vedros (Escola Jos\u00e9 Afonso) - Arroteias", "Longa"],
    "4102": ["Cabe\u00e7o Verde - Sarilhos Pequenos", "Longa"],
    "4103": ["Moita (Esta\u00e7\u00e3o) - Sarilhos Pequenos", "Pr\u00f3xima"],
    "4104": ["Moita | Circular", "Pr\u00f3xima"],
    "4530": ["Bairro Vila Morena - Pinhal Novo", "Longa"],
    "4531": ["Moita - Palmela (Terminal)", "Longa"],
    "4532": ["Moita - Quatro Marcos", "Longa"],
    "4600": ["Alcochete (Freeport) - Barreiro (Terminal)", "Longa"],
    "4601": ["Barreiro (Terminal) - Montijo (Terminal Rodovi\u00e1rio)", "Longa"],
    "4602": ["Alhos Vedros (Esta\u00e7\u00e3o) - Barreiro (Terminal)", "Longa"],
    "4603": ["Barreiro (Terminal) - Ch\u00e3o Duro", "Longa"],
    "4604": ["Barreiro (Terminal) - Moita (Escola Fragata do Tejo)", "Longa"],
    "4605": ["Lavradio - Pinhal do Forno", "Longa"],
    "4610": ["Bairro dos Marinheiros - Barreiro (Terminal)", "Longa"],
    "4611": ["Penalva - Moita (Esc. Secund\u00e1ria)", "Longa"],
    "4620": ["Moita - Paio Pires", "Longa"],
    "4621": ["Moita - Seixal (Terminal Fluvial)", "Longa"],
    "4701": ["Lisboa (Oriente) - Vale da Amoreira", "R\u00e1pida"]
  },
  "Montijo": {
    "4201": ["Afonsoeiro - Bairro da Liberdade", "Longa"],
    "4202": ["Afonsoeiro - Bairro do Saldanha, via Bairro da Cal\u00e7ada", "Longa"],
    "4203": ["Afonsoeiro - Montijo (Terminal Fluvial), via Bairro da Liberdade", "Pr\u00f3xima"],
    "4204": ["Bairro do Charqueir\u00e3o - Montijo (Terminal Fluvial)", "Pr\u00f3xima"],
    "4205": ["Bairro do Charqueir\u00e3o - Montijo (Terminal Fluvial), via Vale Porim", "Longa"],
    "4206": ["Bairro Esteval - Montijo (Terminal Fluvial)", "Pr\u00f3xima"],
    "4207": ["Montijo (\u00c1rea Comercial) - Montijo (Terminal Fluvial)", "Pr\u00f3xima"],
    "4208": ["Montijo (Terminal Rodovi\u00e1rio) - Sarilhos Grandes (Estr. 4 Marcos)", "Longa"],
    "4210": ["Canha - Foros Boavista", "Longa"],
    "4211": ["Craveiras - Peg\u00f5es | Circular", "Pr\u00f3xima"],
    "4212": ["Foros Boavista - Peg\u00f5es", "Longa"],
    "4501": ["Alcochete - Montijo (Terminal Fluvial)", "Longa"],
    "4502": ["Alcochete - Passil", "Longa"],
    "4503": ["Atalaia - Jardia", "Longa"],
    "4504": ["Montijo (Terminal Fluvial) - Passil", "Longa"],
    "4510": ["Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio)", "Longa"],
    "4511": ["Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio), via Samouco", "Longa"],
    "4512": ["Alcochete (Freeport) - Set\u00fabal (ITS), via Alto Estanqueiro", "Longa"],
    "4513": ["Alcochete (Freeport) - Pinhal Novo", "Longa"],
    "4514": ["Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es", "Longa"],
    "4515": ["Montijo (Terminal Rodovi\u00e1rio) - Peg\u00f5es", "Longa"],
    "4516": ["Montijo (Terminal Rodovi\u00e1rio) - Rio Frio", "Longa"],
    "4517": ["Montijo (Terminal Rodovi\u00e1rio) - Set\u00fabal (ITS)", "Longa"],
    "4520": ["Faias - Peg\u00f5es", "Longa"],
    "4521": ["Faias - Pinhal Novo", "Longa"],
    "4522": ["Faias - Poceir\u00e3o", "Longa"],
    "4523": ["Montijo (Terminal Rodovi\u00e1rio) - Pinhal Novo", "Longa"],
    "4524": ["Palmela (Terminal) - Peg\u00f5es", "Longa"],
    "4530": ["Bairro Vila Morena - Pinhal Novo", "Longa"],
    "4600": ["Alcochete (Freeport) - Barreiro (Terminal)", "Longa"],
    "4601": ["Barreiro (Terminal) - Montijo (Terminal Rodovi\u00e1rio)", "Longa"],
    "4643": ["Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)", "Mar"],
    "4701": ["Lisboa (Oriente) - Vale da Amoreira", "R\u00e1pida"],
    "4703": ["Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio), via Alcochete e Samouco", "R\u00e1pida"],
    "4704": ["Atalaia - Lisboa (Oriente)", "R\u00e1pida"],
    "4705": ["Lisboa (Oriente) - Samouco", "R\u00e1pida"],
    "4706": ["S\u00e3o Francisco - Lisboa (Oriente)", "R\u00e1pida"],
    "4707": ["Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio)", "R\u00e1pida"],
    "4902": ["Landeira - Peg\u00f5es", "Longa"],
    "4905": ["Faias - Vendas Novas", "Longa/ Inter-regional"],
    "4906": ["Set\u00fabal (ITS) - Vendas Novas, via Landeira", "Longa/ Inter-regional"]
  },
  "Palmela": {
    "3630": ["Brevemente | Azeit\u00e3o - Penalva (Esta\u00e7\u00e3o)", "Longa"],
    "4301": ["Palmela (Centro) - Palmela (Terminal)", "Pr\u00f3xima"],
    "4302": ["Palmela (Esta\u00e7\u00e3o) - Palmela (Terminal)", "Longa"],
    "4303": ["Palmela | Circular", "Pr\u00f3xima"],
    "4304": ["Palmela (Terminal) - Penalva", "Longa"],
    "4305": ["Brejos do Assa - Palmela (Terminal)", "Longa"],
    "4306": ["Cabanas - Palmela (Terminal)", "Longa"],
    "4307": ["Loja Nova - Palmela (Terminal)", "Longa"],
    "4308": ["Palmela (Terminal) - Pinhal Novo (Esta\u00e7\u00e3o)", "Longa"],
    "4310": ["\u00c1guas de Moura - Poceir\u00e3o", "Longa"],
    "4311": ["Asseiceira - Poceir\u00e3o", "Longa"],
    "4312": ["Poceir\u00e3o - Vale Abrunheira (X), via Fernando P\u00f3", "Longa"],
    "4313": ["Cabanas - Penalva", "Longa"],
    "4320": ["Pinhal Novo | Circular", "Pr\u00f3xima"],
    "4321": ["Pinhal Novo - Qta do Anjo", "Longa"],
    "4322": ["Pinhal Novo - Rio Frio", "Longa"],
    "4512": ["Alcochete (Freeport) - Set\u00fabal (ITS), via Alto Estanqueiro", "Longa"],
    "4513": ["Alcochete (Freeport) - Pinhal Novo", "Longa"],
    "4514": ["Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es", "Longa"],
    "4515": ["Montijo (Terminal Rodovi\u00e1rio) - Peg\u00f5es", "Longa"],
    "4516": ["Montijo (Terminal Rodovi\u00e1rio) - Rio Frio", "Longa"],
    "4517": ["Montijo (Terminal Rodovi\u00e1rio) - Set\u00fabal (ITS)", "Longa"],
    "4520": ["Faias - Peg\u00f5es", "Longa"],
    "4521": ["Faias - Pinhal Novo", "Longa"],
    "4522": ["Faias - Poceir\u00e3o", "Longa"],
    "4523": ["Montijo (Terminal Rodovi\u00e1rio) - Pinhal Novo", "Longa"],
    "4524": ["Palmela (Terminal) - Peg\u00f5es", "Longa"],
    "4530": ["Bairro Vila Morena - Pinhal Novo", "Longa"],
    "4531": ["Moita - Palmela (Terminal)", "Longa"],
    "4532": ["Moita - Quatro Marcos", "Longa"],
    "4540": ["\u00c1guas de Moura - Set\u00fabal (ITS)", "Longa"],
    "4541": ["Algeruz - Set\u00fabal (Av. Lu\u00edsa Todi)", "Longa"],
    "4542": ["Algeruz - Set\u00fabal (ITS)", "Longa"],
    "4543": ["Algeruz - Set\u00fabal (ITS), via Po\u00e7oilos", "Longa"],
    "4544": ["Bairro Marga\u00e7a - Set\u00fabal (ITS)", "Longa"],
    "4545": ["Biscainho - Set\u00fabal (Bela Vista)", "Longa"],
    "4546": ["Biscainho - Set\u00fabal (ITS)", "Longa"],
    "4547": ["Cabanas - Set\u00fabal (ITS)", "Longa"],
    "4548": ["Lagame\u00e7as - Set\u00fabal (ITS)", "Longa"],
    "4549": ["Palmela (Terminal) - Set\u00fabal (ITS)", "Longa"],
    "4550": ["Palmela (Terminal) - Vila Nogueira de Azeit\u00e3o", "Longa"],
    "4551": ["Palmela (USF) - Set\u00fabal (Av. Lu\u00edsa Todi)", "Longa"],
    "4560": ["Cabanas - Vila Nogueira de Azeit\u00e3o", "Longa"],
    "4561": ["Cabanas - Vila Nogueira de Azeit\u00e3o, via Quinta do Pic\u00e3o", "Longa"],
    "4562": ["Set\u00fabal (ITS) - Vila Nogueira de Azeit\u00e3o, via Palmela (Esta\u00e7\u00e3o)", "Longa"],
    "4610": ["Bairro dos Marinheiros - Barreiro (Terminal)", "Longa"],
    "4611": ["Penalva - Moita (Esc. Secund\u00e1ria)", "Longa"],
    "4612": ["Bairro dos Marinheiros - Palmela (Terminal)", "Longa"],
    "4710": ["Lisboa (Oriente) - Palmela (Terminal)", "R\u00e1pida"],
    "4711": ["Lisboa (Oriente) - Pinhal Novo", "R\u00e1pida"],
    "4715": ["Lisboa (Oriente) - Set\u00fabal (ITS), via Pinhal Novo", "R\u00e1pida"],
    "4901": ["Landeira - Set\u00fabal (ITS)", "Longa"],
    "4902": ["Landeira - Peg\u00f5es", "Longa"],
    "4905": ["Faias - Vendas Novas", "Longa/ Inter-regional"],
    "4906": ["Set\u00fabal (ITS) - Vendas Novas, via Landeira", "Longa/ Inter-regional"]
  },
  "Seixal": {
    "3101": ["Amora - Foros de Amora (Esta\u00e7\u00e3o)", "Pr\u00f3xima"],
    "3102": ["Brevemente | Aroeira - Paio Pires (Quinta Flam\u00e2ncia)", "Longa"],
    "3103": ["Corroios (Esta\u00e7\u00e3o) - Paio Pires (Farinheiras)", "Longa"],
    "3104": ["Corroios (Esta\u00e7\u00e3o) - Vale de Milha\u00e7os", "Longa"],
    "3105": ["Fern\u00e3o Ferro - Fogueteiro (Esta\u00e7\u00e3o)", "Longa"],
    "3106": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Fern\u00e3o Ferro", "Pr\u00f3xima"],
    "3107": ["Laranjeiras - Marco do Grilo", "Pr\u00f3xima"],
    "3108": ["Fogueteiro - Foros de Amora (Esta\u00e7\u00e3o)", "Pr\u00f3xima"],
    "3109": ["Fogueteiro (Esta\u00e7\u00e3o) - Parque Empresarial do Seixal", "Longa"],
    "3110": ["Fogueteiro (Esta\u00e7\u00e3o) - Redondos", "Longa"],
    "3111": ["Fogueteiro (Esta\u00e7\u00e3o) - Seixal (Terminal Fluvial)", "Longa"],
    "3112": ["Fogueteiro (Esta\u00e7\u00e3o) - Seixal (Terminal Fluvial), via Paio Pires", "Pr\u00f3xima"],
    "3113": ["Fogueteiro (Esta\u00e7\u00e3o) - Seixal (Terminal Fluvial), via Quinta do Cabral", "Pr\u00f3xima"],
    "3114": ["Foros de Amora - Paio Pires (Quinta Flam\u00e2ncia)", "Longa"],
    "3115": ["Brevemente | Marisol - Foros de Amora (Esta\u00e7\u00e3o), via Corroios (Esta\u00e7\u00e3o)", "Longa"],
    "3116": ["Brevemente | Marisol - Seixal (Terminal Fluvial), via Corroios (Esta\u00e7\u00e3o)", "Longa"],
    "3117": ["Brevemente | Marisol (Valadares) - Foros de Amora (Esta\u00e7\u00e3o)", "Longa"],
    "3118": ["Brevemente | Marisol (Valadares) - Seixal (Terminal Fluvial)", "Longa"],
    "3119": ["Pinhal Conde Cunha - Seixal (Terminal Fluvial)", "Longa"],
    "3120": ["Redondos - Seixal (Terminal Fluvial)", "Longa"],
    "3121": ["Seixal | Circular", "Pr\u00f3xima"],
    "3122": ["Brevemente | Verdizela - Cruz de Pau", "Longa"],
    "3501": ["Brevemente | Almada Forum - Marisol, via Sobreda", "Longa"],
    "3502": ["Almada Forum - Paio Pires (Centro)", "Longa"],
    "3503": ["Brevemente | Almada Forum - Vale de Milha\u00e7os", "Longa"],
    "3504": ["Brevemente | Bairro Fundo Fomento - Quintinha", "Longa"],
    "3505": ["Brevemente | Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o)", "Longa"],
    "3506": ["Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o), via Miratejo", "Longa"],
    "3507": ["Cacilhas (Terminal) - Marisol", "Longa"],
    "3508": ["Cacilhas (Terminal) - Paio Pires (Centro)", "Longa"],
    "3509": ["Cacilhas (Terminal) - Paio Pires (Centro), via Seixal (Terminal Fluvial) e Amora", "Longa"],
    "3510": ["Cacilhas (Terminal) - Pilotos", "Longa"],
    "3511": ["Cacilhas (Terminal) - Pinheirinho", "Longa"],
    "3512": ["Cacilhas (Terminal) - Quinta Princesa", "Longa"],
    "3513": ["Cacilhas (Terminal) - Santa Marta do Pinhal", "Longa"],
    "3514": ["Cacilhas (Terminal) - Vale de Milha\u00e7os", "Longa"],
    "3515": ["Caparica (Pilotos) -  Corroios", "Longa"],
    "3516": ["Charneca da Caparica - Corroios (Esta\u00e7\u00e3o)", "Longa"],
    "3517": ["Chegadinho - Corroios (Esta\u00e7\u00e3o)", "Longa"],
    "3518": ["Corroios (Esta\u00e7\u00e3o) - Vale de Figueira", "Longa"],
    "3519": ["Costa da Caparica (Terminal) - Corroios (Esta\u00e7\u00e3o)", "Longa"],
    "3520": ["Costa da Caparica (Terminal) - Quinta do Brasileiro", "Longa"],
    "3521": ["Cruz de Pau - Fonta da Telha", "Longa"],
    "3522": ["Fonte da Telha - Paio Pires (Centro)", "Longa"],
    "3523": ["Fonte da Telha - Paio Pires (Quinta Flam\u00e2ncia), via Seixal (Terminal Fluvial) e Foros de Amora (Esta\u00e7\u00e3o)", "Longa"],
    "3524": ["Hospital Garcia de Orta - Marisol", "Longa"],
    "3525": ["Hospital Garcia de Orta - Miratejo", "Longa"],
    "3526": ["Laranjeiro - Pinheirinho", "Longa"],
    "3527": ["Monte da Caparica (FCT) - Paio Pires (Bairro Cucena)", "Longa"],
    "3528": ["Monte da Caparica (FCT) - Paio Pires (Centro)", "Longa"],
    "3535": ["Cacilhas (Terminal) - Quinta do Conde", "Longa"],
    "3536": ["Cacilhas (Terminal) - Sesimbra (Terminal)", "Longa"],
    "3540": ["Brevemente | Alfarim - Coina (Esta\u00e7\u00e3o)", "Longa"],
    "3541": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Fern\u00e3o Ferro, via Casal do Sapo e Pinhal do General", "Longa"],
    "3542": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Praia do Meco", "Longa"],
    "3543": ["Coina (Esta\u00e7\u00e3o) - Quinta do Conde, via Estrada de Coina", "Longa"],
    "3544": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal)", "Longa"],
    "3545": ["Corroios (Esta\u00e7\u00e3o) - Sesimbra (Terminal)", "Longa"],
    "3546": ["Cruz de Pau - Quinta do Conde", "Longa"],
    "3547": ["Fogueteiro (Esta\u00e7\u00e3o) - Quinta do Conde", "Longa"],
    "3548": ["Marco do Grilo - Quinta do Conde", "Longa"],
    "3549": ["Quinta do Conde - Sesimbra (Terminal), via Sampaio e Marco do Grilo", "Longa"],
    "3601": ["Barreiro - Cova da Piedade (Metro)", "Longa"],
    "3605": ["Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o", "Longa"],
    "3610": ["Brevemente | Cacilhas (Terminal) - Set\u00fabal (ITS), via A2", "Longa"],
    "3615": ["Barreiro - Seixal", "Longa"],
    "3620": ["Coina (Esta\u00e7\u00e3o) - Quinta do Conde", "Longa"],
    "3625": ["Brevemente | Barreiro - Sesimbra (Terminal)", "Longa"],
    "3626": ["Coina (Esta\u00e7\u00e3o) - Vila Fresca de Azeit\u00e3o", "Longa"],
    "3635": ["Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal), via Azeit\u00e3o e Sampaio", "Longa"],
    "3715": ["Lisboa (M. Pombal) - Santa Marta do Pinhal", "Longa"],
    "3716": ["Lisboa (Sete Rios) - Marisol", "Longa"],
    "3717": ["Lisboa (Sete Rios) - Quinta do Brasileiro", "Longa"],
    "3720": ["Lisboa (Sete Rios) - Quinta do Conde", "R\u00e1pida"],
    "3721": ["Lisboa (Sete Rios) - Sesimbra (Terminal)", "R\u00e1pida"],
    "4620": ["Moita - Paio Pires", "Longa"],
    "4621": ["Moita - Seixal (Terminal Fluvial)", "Longa"],
    "4630": ["Corroios (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
    "4631": ["Fogueteiro (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"]
  },
  "Sesimbra": {
    "3201": ["Aldeia do Meco - Sesimbra (Terminal), via Aiana", "Longa"],
    "3202": ["Alfarim - Sesimbra (Terminal), via Aiana", "Longa"],
    "3203": ["Azoia - Sesimbra (Terminal)", "Longa"],
    "3204": ["Azoia - Sesimbra (Terminal), via Serra da Azoia", "Longa"],
    "3205": ["Cabo Espichel - Sesimbra (Terminal)", "Longa"],
    "3206": ["Carrasqueira - Sesimbra (Terminal)", "Longa"],
    "3207": ["Carrasqueira - Sesimbra (Terminal), via Valbom e Sampaio", "Longa"],
    "3208": ["Fetais - Sesimbra (Terminal)", "Longa"],
    "3209": ["Fornos - Sesimbra (Terminal), via Aiana", "Longa"],
    "3210": ["Lagoa de Albufeira - Sesimbra (Terminal)", "Longa"],
    "3211": ["Lagoa de Albufeira - Sesimbra (Terminal), via Sampaio", "Longa"],
    "3212": ["Ma\u00e7\u00e3 (Rua Macieira) - Sesimbra (Terminal)", "Longa"],
    "3213": ["Pinhal de Cima - Sesimbra (Terminal) | Circular", "Longa"],
    "3214": ["Sampaio - Santana", "Longa"],
    "3215": ["Fornos - Sampaio", "Longa"],
    "3216": ["Alto das Vinhas - Sampaio", "Longa"],
    "3217": ["Azoia - Sampaio", "Longa"],
    "3218": ["Sesimbra (Porto de Abrigo) - Sesimbra (Terminal)", "Pr\u00f3xima"],
    "3219": ["Brevemente | Sesimbra (R. Palames) - Sesimbra (Terminal)", "Pr\u00f3xima"],
    "3220": ["Sesimbra | Circular", "Pr\u00f3xima"],
    "3221": ["Valbom - Sesimbra (Terminal), via Sampaio", "Longa"],
    "3222": ["Quinta do Conde | Circular", "Pr\u00f3xima"],
    "3535": ["Cacilhas (Terminal) - Quinta do Conde", "Longa"],
    "3536": ["Cacilhas (Terminal) - Sesimbra (Terminal)", "Longa"],
    "3540": ["Brevemente | Alfarim - Coina (Esta\u00e7\u00e3o)", "Longa"],
    "3541": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Fern\u00e3o Ferro, via Casal do Sapo e Pinhal do General", "Longa"],
    "3542": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Praia do Meco", "Longa"],
    "3543": ["Coina (Esta\u00e7\u00e3o) - Quinta do Conde, via Estrada de Coina", "Longa"],
    "3544": ["Brevemente | Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal)", "Longa"],
    "3545": ["Corroios (Esta\u00e7\u00e3o) - Sesimbra (Terminal)", "Longa"],
    "3546": ["Cruz de Pau - Quinta do Conde", "Longa"],
    "3547": ["Fogueteiro (Esta\u00e7\u00e3o) - Quinta do Conde", "Longa"],
    "3548": ["Marco do Grilo - Quinta do Conde", "Longa"],
    "3549": ["Quinta do Conde - Sesimbra (Terminal), via Sampaio e Marco do Grilo", "Longa"],
    "3605": ["Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o", "Longa"],
    "3620": ["Coina (Esta\u00e7\u00e3o) - Quinta do Conde", "Longa"],
    "3625": ["Brevemente | Barreiro - Sesimbra (Terminal)", "Longa"],
    "3626": ["Coina (Esta\u00e7\u00e3o) - Vila Fresca de Azeit\u00e3o", "Longa"],
    "3630": ["Brevemente | Azeit\u00e3o - Penalva (Esta\u00e7\u00e3o)", "Longa"],
    "3635": ["Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal), via Azeit\u00e3o e Sampaio", "Longa"],
    "3640": ["Azoia - Vila Nogueira de Azeit\u00e3o", "Longa"],
    "3641": ["Quinta do Conde - Sampaio, via Azeit\u00e3o", "Longa"],
    "3650": ["Brevemente | Moita - Sesimbra (Terminal)", "Mar"],
    "3720": ["Lisboa (Sete Rios) - Quinta do Conde", "R\u00e1pida"],
    "3721": ["Lisboa (Sete Rios) - Sesimbra (Terminal)", "R\u00e1pida"],
    "4630": ["Corroios (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
    "4631": ["Fogueteiro (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
    "4640": ["Casais da Serra - Vila Nogueira de Azeit\u00e3o", "Longa"],
    "4641": ["Quinta do Conde - Set\u00fabal (ITS)", "Longa"],
    "4642": ["Sesimbra (Terminal) - Set\u00fabal (Hospital)", "Longa"],
    "4643": ["Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)", "Mar"]
  },
  "Set\u00fabal": {
    "3605": ["Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o", "Longa"],
    "3610": ["Brevemente | Cacilhas (Terminal) - Set\u00fabal (ITS), via A2", "Longa"],
    "3625": ["Brevemente | Barreiro - Sesimbra (Terminal)", "Longa"],
    "3626": ["Coina (Esta\u00e7\u00e3o) - Vila Fresca de Azeit\u00e3o", "Longa"],
    "3630": ["Brevemente | Azeit\u00e3o - Penalva (Esta\u00e7\u00e3o)", "Longa"],
    "3635": ["Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal), via Azeit\u00e3o e Sampaio", "Longa"],
    "3640": ["Azoia - Vila Nogueira de Azeit\u00e3o", "Longa"],
    "3641": ["Quinta do Conde - Sampaio, via Azeit\u00e3o", "Longa"],
    "4401": ["Cachofarra - Set\u00fabal (Hospital)", "Longa"],
    "4402": ["Estefanilha - Set\u00fabal (ITS)", "Longa"],
    "4403": ["Fonte da Talha - Set\u00fabal (Av. Lu\u00edsa Todi)", "Pr\u00f3xima"],
    "4404": ["Interfaces Set\u00fabal | Circular", "Pr\u00f3xima"],
    "4405": ["Livramento-Montebelo | Circular", "Longa"],
    "4406": ["Manteigadas - Set\u00fabal (Mercado)", "Pr\u00f3xima"],
    "4407": ["Manteigadas - Set\u00fabal (Mercado), via Bairro da Carmona", "Longa"],
    "4408": ["Manteigadas - Set\u00fabal (Mercado), via Bela Vista", "Longa"],
    "4409": ["Manteigadas - Viso", "Longa"],
    "4410": ["Manteigadas (Esc. Profissional) - Set\u00fabal (Alegro)", "Longa"],
    "4411": ["Morgada - Set\u00fabal (ITS)", "Longa"],
    "4412": ["Morgada - Set\u00fabal (Mercado)", "Longa"],
    "4413": ["Morgada - Set\u00fabal (Mercado), via Bela Vista", "Longa"],
    "4414": ["Out\u00e3o (Hospital) - Set\u00fabal (ITS)", "Longa"],
    "4415": ["Out\u00e3o (Hospital) - Set\u00fabal (ITS), via vale da Rasca", "Longa"],
    "4416": ["Po\u00e7o Mouro - Set\u00fabal (ITS)", "Longa"],
    "4417": ["Po\u00e7o Mouro - Set\u00fabal (ITS), via Manteigadas", "Longa"],
    "4418": ["Set\u00fabal (Alegro) - Set\u00fabal (Av. 5 Outubro)", "Longa"],
    "4419": ["Brejos Canes - Set\u00fabal (Saboaria)", "Longa"],
    "4420": ["Set\u00fabal (Alegro) - Set\u00fabal (ITS)", "Pr\u00f3xima"],
    "4421": ["Set\u00fabal (Bairro Camolas) - Set\u00fabal (Casal Figueiras)", "Longa"],
    "4422": ["Set\u00fabal (Bairro Camolas) - Set\u00fabal (Casal Figueiras), via Bairro do Viso", "Longa"],
    "4423": ["Amoreiras \u2013 Set\u00fabal (Av. Lu\u00edsa Todi)", "Pr\u00f3xima"],
    "4424": ["Set\u00fabal (Bairro Viso) - Manteigadas", "Longa"],
    "4425": ["Set\u00fabal (Escola Viso) - Mitrena", "Longa"],
    "4426": ["Set\u00fabal (Bairro Viso) - Set\u00fabal (CHEset\u00fabal)", "Pr\u00f3xima"],
    "4427": ["Set\u00fabal (Bela Vista) - Set\u00fabal (Mercado)", "Pr\u00f3xima"],
    "4428": ["Set\u00fabal (Casal Figueiras) - Vale Ana Gomes", "Longa"],
    "4429": ["Set\u00fabal (Centro Sa\u00fade) - Set\u00fabal (Mercado)", "Longa"],
    "4430": ["Set\u00fabal (Hospital) - Set\u00fabal (Montalv\u00e3o)", "Pr\u00f3xima"],
    "4431": ["Set\u00fabal (ITS) - Set\u00fabal (Quinta Varzinha)", "Pr\u00f3xima"],
    "4432": ["Set\u00fabal (ITS) - Vale de Choupo", "Longa"],
    "4433": ["Alto Guerra - Set\u00fabal (Casal Figueiras)", "Longa"],
    "4434": ["Set\u00fabal (Mercado 2 de Abril) - Set\u00fabal (R. Timor)", "Pr\u00f3xima"],
    "4435": ["Biscainho - Faralh\u00e3o", "Longa"],
    "4436": ["Set\u00fabal (Mercado) - Set\u00fabal (Av. Soeiro Pereira Gomes)", "Pr\u00f3xima"],
    "4437": ["Faralh\u00e3o - Set\u00fabal (ITS)", "Longa"],
    "4438": ["Set\u00fabal (Monte Belo Norte) - Set\u00fabal (Saboaria)", "Longa"],
    "4439": ["Praias do Sado - Set\u00fabal (ITS)", "Longa"],
    "4440": ["Set\u00fabal (Monte Belo Norte) - Set\u00fabal (Saboaria), via Alegro", "Longa"],
    "4441": ["Set\u00fabal (Saboaria) - Set\u00fabal (Vale Cobro)", "Longa"],
    "4442": ["Praias do Sado (Esta\u00e7\u00e3o) - Set\u00fabal (Bela Vista)", "Longa"],
    "4443": ["Set\u00fabal (Polit\u00e9cnico) - Praias do Sado", "Longa"],
    "4451": ["Mitrena (Lisnave) - Set\u00fabal (ITS)", "Longa"],
    "4452": ["Mitrena (Portucel) - Set\u00fabal (ITS)", "Longa"],
    "4453": ["Mitrena (Portucel) - Set\u00fabal (ITS), via Estrada Gra\u00e7a", "Longa"],
    "4460": ["Azeit\u00e3o | Circular", "Longa"],
    "4470": ["Brejos Azeit\u00e3o - Praia do Creiro", "Longa"],
    "4471": ["Praia Albarquel | Circular", "Longa"],
    "4472": ["Praia do Creiro - Set\u00fabal (ITS)", "Longa"],
    "4474": ["Figueirinha - Set\u00fabal (Alegro)", "Longa"],
    "4475": ["Portinho da Arr\u00e1bida - Viso", "Longa"],
    "4476": ["Praias Arr\u00e1bida | Circular", "Longa"],
    "4512": ["Alcochete (Freeport) - Set\u00fabal (ITS), via Alto Estanqueiro", "Longa"],
    "4517": ["Montijo (Terminal Rodovi\u00e1rio) - Set\u00fabal (ITS)", "Longa"],
    "4540": ["\u00c1guas de Moura - Set\u00fabal (ITS)", "Longa"],
    "4541": ["Algeruz - Set\u00fabal (Av. Lu\u00edsa Todi)", "Longa"],
    "4542": ["Algeruz - Set\u00fabal (ITS)", "Longa"],
    "4543": ["Algeruz - Set\u00fabal (ITS), via Po\u00e7oilos", "Longa"],
    "4544": ["Bairro Marga\u00e7a - Set\u00fabal (ITS)", "Longa"],
    "4545": ["Biscainho - Set\u00fabal (Bela Vista)", "Longa"],
    "4546": ["Biscainho - Set\u00fabal (ITS)", "Longa"],
    "4547": ["Cabanas - Set\u00fabal (ITS)", "Longa"],
    "4548": ["Lagame\u00e7as - Set\u00fabal (ITS)", "Longa"],
    "4549": ["Palmela (Terminal) - Set\u00fabal (ITS)", "Longa"],
    "4550": ["Palmela (Terminal) - Vila Nogueira de Azeit\u00e3o", "Longa"],
    "4551": ["Palmela (USF) - Set\u00fabal (Av. Lu\u00edsa Todi)", "Longa"],
    "4560": ["Cabanas - Vila Nogueira de Azeit\u00e3o", "Longa"],
    "4561": ["Cabanas - Vila Nogueira de Azeit\u00e3o, via Quinta do Pic\u00e3o", "Longa"],
    "4562": ["Set\u00fabal (ITS) - Vila Nogueira de Azeit\u00e3o, via Palmela (Esta\u00e7\u00e3o)", "Longa"],
    "4630": ["Corroios (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
    "4631": ["Fogueteiro (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)", "Longa"],
    "4640": ["Casais da Serra - Vila Nogueira de Azeit\u00e3o", "Longa"],
    "4641": ["Quinta do Conde - Set\u00fabal (ITS)", "Longa"],
    "4642": ["Sesimbra (Terminal) - Set\u00fabal (Hospital)", "Longa"],
    "4643": ["Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)", "Mar"],
    "4715": ["Lisboa (Oriente) - Set\u00fabal (ITS), via Pinhal Novo", "R\u00e1pida"],
    "4720": ["Lisboa (Oriente) - Set\u00fabal (ITS)", "R\u00e1pida"],
    "4725": ["Lisboa (Sete Rios) - Set\u00fabal (ITS)", "R\u00e1pida"],
    "4901": ["Landeira - Set\u00fabal (ITS)", "Longa"],
    "4906": ["Set\u00fabal (ITS) - Vendas Novas, via Landeira", "Longa/ Inter-regional"]
  },
  "Vendas Novas (CIMAC)": {
    "4901": ["Landeira - Set\u00fabal (ITS)", "Longa"],
    "4902": ["Landeira - Peg\u00f5es", "Longa"],
    "4905": ["Faias - Vendas Novas", "Longa/ Inter-regional"],
    "4906": ["Set\u00fabal (ITS) - Vendas Novas, via Landeira", "Longa/ Inter-regional"]
  }
};
const diretorio = {
  "Alcochete": {
    "410": [["4511", "Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio), via Samouco"], ["4600", "Alcochete (Freeport) - Barreiro (Terminal)"]],
    "412 (Adaptado)": [["4501", "Alcochete - Montijo (Terminal Fluvial)"]],
    "413": [["4510", "Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio)"], ["4512", "Alcochete (Freeport) - Set\u00fabal (ITS), via Alto Estanqueiro"], ["4513", "Alcochete (Freeport) - Pinhal Novo"]],
    "414": [["4514", "Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es"]],
    "415": [["4504", "Montijo (Terminal Fluvial) - Passil"]],
    "416": [["4514", "Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es"], ["4515", "Montijo (Terminal Rodovi\u00e1rio) - Peg\u00f5es"]],
    "419": [["4502", "Alcochete - Passil"]],
    "426": [["4516", "Montijo (Terminal Rodovi\u00e1rio) - Rio Frio"]],
    "431": [["4703", "Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio), via Alcochete e Samouco"]],
    "432": [["4702", "Lisboa (Oriente) - Valbom"], ["4704", "Atalaia - Lisboa (Oriente)"]],
    "435": [["4705", "Lisboa (Oriente) - Samouco"]],
    "453": [["4706", "S\u00e3o Francisco - Lisboa (Oriente)"]],
    "Nova": [["4001", "Alcochete | Circular"], ["4002", "S\u00e3o Francisco | Circular"], ["4503", "Atalaia - Jardia"]]
  },
  "Almada": {
    "101": [["3001", "Almada (Cristo Rei) - Cacilhas (Terminal)"]],
    "102": [["3003", "Almada Forum - Cacilhas (Terminal)"]],
    "106": [["3007", "Bairro Fundo Fomento - Cacilhas (Terminal)"]],
    "110": [["3513", "Cacilhas (Terminal) - Santa Marta do Pinhal"]],
    "114": [["3508", "Cacilhas (Terminal) - Paio Pires (Centro)"]],
    "116": [["3521", "Cruz de Pau - Fonta da Telha"], ["3523", "Fonte da Telha - Paio Pires (Quinta Flam\u00e2ncia), via Seixal (Terminal Fluvial) e Foros de Amora (Esta\u00e7\u00e3o)"]],
    "117": [["3014", "Cacilhas (terminal) - Raposeira"]],
    "120": [["3515", "Caparica (Pilotos) -  Corroios"]],
    "121": [["3526", "Laranjeiro - Pinheirinho"]],
    "123 (Adaptado)": [["3013", "Cacilhas (Terminal) - Monte da Caparica"]],
    "124 (Adaptado)": [["3022", "Costa da Caparica (Terminal) - Hospital Garcia de Orta"]],
    "126 (Adaptado)": [["3507", "Cacilhas (Terminal) - Marisol"], ["3524", "Hospital Garcia de Orta - Marisol"]],
    "127 (Adaptado)": [["3012", "Cacilhas (Terminal) - Fonte da Telha"]],
    "129 (Adaptado)": [["3030", "Fonte da Telha - Monte da Caparica (FCT)"]],
    "130 (Adaptado)": [["3030", "Fonte da Telha - Monte da Caparica (FCT)"]],
    "135": [["3011", "Cacilhas (Terminal) - Costa da Caparica"]],
    "139": [["3519", "Costa da Caparica (Terminal) - Corroios (Esta\u00e7\u00e3o)"]],
    "143": [["3518", "Corroios (Esta\u00e7\u00e3o) - Vale de Figueira"]],
    "145 (Adaptado)": [["3010", "Cacilhas (Terminal) - Charneca da Caparica"]],
    "146": [["3008", "Ban\u00e1tica - Quintinha"]],
    "146 (Adaptado)": [["3032", "Brevemente | Monte da Caparica (FCT) - Quinta do Texugo"], ["3034", "Porto Brand\u00e3o (Terminal) - Quinta do Texugo"]],
    "149": [["3512", "Cacilhas (Terminal) - Quinta Princesa"]],
    "151": [["3704", "Charneca da Caparica - Lisboa (M. Pombal)"]],
    "153": [["3710", "Costa da Caparica (Terminal) - Lisboa (Sete Rios)"]],
    "155": [["3709", "Costa da Caparica (Terminal) - Lisboa (M. Pombal)"]],
    "158": [["3711", "Monte da Caparica (FCT) - Lisboa (Sete Rios)"]],
    "159": [["3707", "Charneca da Caparica - Lisboa (Sete Rios), via Sobreda"], ["3716", "Lisboa (Sete Rios) - Marisol"]],
    "159 (adaptada)": [["3501", "Brevemente | Almada Forum - Marisol, via Sobreda"]],
    "160": [["3703", "Almada (Parque Urbano) - Lisboa (Sete Rios)"]],
    "161": [["3710", "Costa da Caparica (Terminal) - Lisboa (Sete Rios)"]],
    "162": [["3717", "Lisboa (Sete Rios) - Quinta do Brasileiro"]],
    "163": [["3520", "Costa da Caparica (Terminal) - Quinta do Brasileiro"]],
    "167": [["3023", "Brevemente | Costa da Caparica (terminal) - Laranjeiro"]],
    "169": [["3715", "Lisboa (M. Pombal) - Santa Marta do Pinhal"]],
    "171": [["3015", "Charneca da Caparica - Cova do Vapor"]],
    "172": [["3522", "Fonte da Telha - Paio Pires (Centro)"]],
    "174 (Adaptado)": [["3024", "Costa da Caparica (Terminal) - Pragal (Esta\u00e7\u00e3o)"]],
    "175 (adaptada)": [["3501", "Brevemente | Almada Forum - Marisol, via Sobreda"]],
    "176": [["3702", "Almada (Parque Urbano) - Lisboa (C. Universit\u00e1ria)"]],
    "179(adaptada)": [["3004", "Almada Forum - Marisol"]],
    "180": [["3018", "Brevemente | Charneca da Caparica - Sobreda"]],
    "181 (adaptada)": [["3525", "Hospital Garcia de Orta - Miratejo"]],
    "182": [["3026", "Cova da Piedade - Hospital Garcia de Orta"]],
    "190 (adaptada)": [["3705", "Brevemente | Charneca da Caparica - Lisboa (Sete Rios)"]],
    "191": [["3514", "Cacilhas (Terminal) - Vale de Milha\u00e7os"]],
    "192": [["3511", "Cacilhas (Terminal) - Pinheirinho"]],
    "196": [["3510", "Cacilhas (Terminal) - Pilotos"]],
    "197": [["3504", "Brevemente | Bairro Fundo Fomento - Quintinha"]],
    "198": [["3502", "Almada Forum - Paio Pires (Centro)"], ["3527", "Monte da Caparica (FCT) - Paio Pires (Bairro Cucena)"], ["3528", "Monte da Caparica (FCT) - Paio Pires (Centro)"]],
    "199": [["3509", "Cacilhas (Terminal) - Paio Pires (Centro), via Seixal (Terminal Fluvial) e Amora"]],
    "1C": [["3516", "Charneca da Caparica - Corroios (Esta\u00e7\u00e3o)"]],
    "1P": [["3027", "Hospital Garcia de Orta - Sobreda"]],
    "203": [["3536", "Cacilhas (Terminal) - Sesimbra (Terminal)"]],
    "207": [["3721", "Lisboa (Sete Rios) - Sesimbra (Terminal)"]],
    "252": [["3720", "Lisboa (Sete Rios) - Quinta do Conde"]],
    "254": [["3535", "Cacilhas (Terminal) - Quinta do Conde"]],
    "2C (Adaptado)": [["3506", "Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o), via Miratejo"]],
    "3C": [["3517", "Chegadinho - Corroios (Esta\u00e7\u00e3o)"]],
    "561": [["4725", "Lisboa (Sete Rios) - Set\u00fabal (ITS)"]],
    "583": [["3610", "Brevemente | Cacilhas (Terminal) - Set\u00fabal (ITS), via A2"]],
    "783": [["3605", "Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o"]],
    "Flexibus": [["3005", "Brevemente | Flexibus Almada | Circular"]],
    "Nova": [["3002", "Brevemente | Almada (Parque Urbano) - Pragal (Esta\u00e7\u00e3o)"], ["3006", "Brevemente | Aroeira | Circular"], ["3009", "Cacilhas (terminal - Trafaria (Terminal)"], ["3016", "Brevemente | Charneca da Caparica - Lazarim"], ["3017", "Charneca da Caparica - Pragal (Esta\u00e7\u00e3o)"], ["3019", "Charneca da Caparica - Trafaria (Terminal)"], ["3020", "Brevemente | Charneca da Caparica | Circular"], ["3021", "Costa da Caparica - Monte da Caparica (FCT)"], ["3025", "Brevemente | Costa da Caparica (Terminal) - Pragal (Esta\u00e7\u00e3o), via IC20"], ["3028", "Brevemente | Lazarim | Circular"], ["3029", "Brevemente | Marco Caba\u00e7o | Circular"], ["3031", "Brevemente | Monte da Caparica - Quintinha"], ["3033", "Brevemente | Monte da Caparica | Circular"], ["3035", "Pragal (Esta\u00e7\u00e3o) - Quinta do Texugo"], ["3036", "Pragal (Esta\u00e7\u00e3o) - Vale Flores"], ["3037", "Brevemente | Quintinha | Circular"], ["3503", "Brevemente | Almada Forum - Vale de Milha\u00e7os"], ["3505", "Brevemente | Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o)"], ["3601", "Barreiro - Cova da Piedade (Metro)"], ["3701", "Brevemente | Almada (Centro Sul) - Alg\u00e9s (Terminal)"], ["3706", "Brevemente | Charneca da Caparica - Lisboa (Sete Rios), via Feij\u00f3"], ["3708", "Brevemente | Costa da Caparica (Terminal) - Lisboa (C. Sodr\u00e9)"]]
  },
  "Barreiro": {
    "1N": [["3620", "Coina (Esta\u00e7\u00e3o) - Quinta do Conde"]],
    "245": [["3650", "Brevemente | Moita - Sesimbra (Terminal)"]],
    "2N": [["3626", "Coina (Esta\u00e7\u00e3o) - Vila Fresca de Azeit\u00e3o"]],
    "302": [["4620", "Moita - Paio Pires"]],
    "305": [["4610", "Bairro dos Marinheiros - Barreiro (Terminal)"]],
    "307 (Adaptado)": [["4604", "Barreiro (Terminal) - Moita (Escola Fragata do Tejo)"]],
    "311 (Adaptado)": [["4611", "Penalva - Moita (Esc. Secund\u00e1ria)"]],
    "317": [["4602", "Alhos Vedros (Esta\u00e7\u00e3o) - Barreiro (Terminal)"]],
    "330": [["4605", "Lavradio - Pinhal do Forno"]],
    "410": [["4600", "Alcochete (Freeport) - Barreiro (Terminal)"], ["4601", "Barreiro (Terminal) - Montijo (Terminal Rodovi\u00e1rio)"], ["4603", "Barreiro (Terminal) - Ch\u00e3o Duro"]],
    "754": [["4631", "Fogueteiro (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)"]],
    "755": [["4630", "Corroios (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)"]],
    "783": [["3605", "Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o"]],
    "Nova": [["3601", "Barreiro - Cova da Piedade (Metro)"], ["3615", "Barreiro - Seixal"], ["3625", "Brevemente | Barreiro - Sesimbra (Terminal)"], ["4612", "Bairro dos Marinheiros - Palmela (Terminal)"], ["4621", "Moita - Seixal (Terminal Fluvial)"]]
  },
  "Lisboa": {
    "151": [["3704", "Charneca da Caparica - Lisboa (M. Pombal)"]],
    "153": [["3710", "Costa da Caparica (Terminal) - Lisboa (Sete Rios)"]],
    "155": [["3709", "Costa da Caparica (Terminal) - Lisboa (M. Pombal)"]],
    "158": [["3711", "Monte da Caparica (FCT) - Lisboa (Sete Rios)"]],
    "159": [["3707", "Charneca da Caparica - Lisboa (Sete Rios), via Sobreda"], ["3716", "Lisboa (Sete Rios) - Marisol"]],
    "160": [["3703", "Almada (Parque Urbano) - Lisboa (Sete Rios)"]],
    "161": [["3710", "Costa da Caparica (Terminal) - Lisboa (Sete Rios)"]],
    "162": [["3717", "Lisboa (Sete Rios) - Quinta do Brasileiro"]],
    "169": [["3715", "Lisboa (M. Pombal) - Santa Marta do Pinhal"]],
    "176": [["3702", "Almada (Parque Urbano) - Lisboa (C. Universit\u00e1ria)"]],
    "190 (adaptada)": [["3705", "Brevemente | Charneca da Caparica - Lisboa (Sete Rios)"]],
    "207": [["3721", "Lisboa (Sete Rios) - Sesimbra (Terminal)"]],
    "252": [["3720", "Lisboa (Sete Rios) - Quinta do Conde"]],
    "333": [["4701", "Lisboa (Oriente) - Vale da Amoreira"]],
    "431": [["4703", "Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio), via Alcochete e Samouco"]],
    "432": [["4702", "Lisboa (Oriente) - Valbom"], ["4704", "Atalaia - Lisboa (Oriente)"]],
    "435": [["4705", "Lisboa (Oriente) - Samouco"], ["4707", "Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio)"]],
    "453": [["4706", "S\u00e3o Francisco - Lisboa (Oriente)"]],
    "561": [["4725", "Lisboa (Sete Rios) - Set\u00fabal (ITS)"]],
    "562": [["4720", "Lisboa (Oriente) - Set\u00fabal (ITS)"]],
    "563": [["4715", "Lisboa (Oriente) - Set\u00fabal (ITS), via Pinhal Novo"]],
    "565": [["4710", "Lisboa (Oriente) - Palmela (Terminal)"], ["4711", "Lisboa (Oriente) - Pinhal Novo"]],
    "Nova": [["3701", "Brevemente | Almada (Centro Sul) - Alg\u00e9s (Terminal)"], ["3706", "Brevemente | Charneca da Caparica - Lisboa (Sete Rios), via Feij\u00f3"], ["3708", "Brevemente | Costa da Caparica (Terminal) - Lisboa (C. Sodr\u00e9)"]]
  },
  "Moita": {
    "245": [["3650", "Brevemente | Moita - Sesimbra (Terminal)"]],
    "302": [["4620", "Moita - Paio Pires"]],
    "305": [["4610", "Bairro dos Marinheiros - Barreiro (Terminal)"]],
    "307 (Adaptado)": [["4604", "Barreiro (Terminal) - Moita (Escola Fragata do Tejo)"]],
    "311 (Adaptado)": [["4611", "Penalva - Moita (Esc. Secund\u00e1ria)"]],
    "312 (Adaptado)": [["4102", "Cabe\u00e7o Verde - Sarilhos Pequenos"]],
    "313 (Adaptado)": [["4104", "Moita | Circular"], ["4532", "Moita - Quatro Marcos"]],
    "317": [["4602", "Alhos Vedros (Esta\u00e7\u00e3o) - Barreiro (Terminal)"]],
    "318": [["4103", "Moita (Esta\u00e7\u00e3o) - Sarilhos Pequenos"]],
    "326 (Adaptado)": [["4531", "Moita - Palmela (Terminal)"]],
    "330": [["4605", "Lavradio - Pinhal do Forno"]],
    "333": [["4701", "Lisboa (Oriente) - Vale da Amoreira"]],
    "336 (Adaptado)": [["4101", "Alhos Vedros (Escola Jos\u00e9 Afonso) - Arroteias"]],
    "410": [["4600", "Alcochete (Freeport) - Barreiro (Terminal)"], ["4601", "Barreiro (Terminal) - Montijo (Terminal Rodovi\u00e1rio)"], ["4603", "Barreiro (Terminal) - Ch\u00e3o Duro"]],
    "451": [["4530", "Bairro Vila Morena - Pinhal Novo"]],
    "Nova": [["4621", "Moita - Seixal (Terminal Fluvial)"]]
  },
  "Montijo": {
    "333": [["4701", "Lisboa (Oriente) - Vale da Amoreira"]],
    "401": [["4204", "Bairro do Charqueir\u00e3o - Montijo (Terminal Fluvial)"], ["4205", "Bairro do Charqueir\u00e3o - Montijo (Terminal Fluvial), via Vale Porim"], ["4207", "Montijo (\u00c1rea Comercial) - Montijo (Terminal Fluvial)"]],
    "403": [["4203", "Afonsoeiro - Montijo (Terminal Fluvial), via Bairro da Liberdade"], ["4206", "Bairro Esteval - Montijo (Terminal Fluvial)"]],
    "404": [["4202", "Afonsoeiro - Bairro do Saldanha, via Bairro da Cal\u00e7ada"]],
    "410": [["4511", "Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio), via Samouco"], ["4600", "Alcochete (Freeport) - Barreiro (Terminal)"], ["4601", "Barreiro (Terminal) - Montijo (Terminal Rodovi\u00e1rio)"]],
    "412 (Adaptado)": [["4501", "Alcochete - Montijo (Terminal Fluvial)"]],
    "413": [["4510", "Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio)"], ["4512", "Alcochete (Freeport) - Set\u00fabal (ITS), via Alto Estanqueiro"], ["4513", "Alcochete (Freeport) - Pinhal Novo"], ["4517", "Montijo (Terminal Rodovi\u00e1rio) - Set\u00fabal (ITS)"], ["4523", "Montijo (Terminal Rodovi\u00e1rio) - Pinhal Novo"]],
    "414": [["4514", "Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es"]],
    "415": [["4504", "Montijo (Terminal Fluvial) - Passil"]],
    "416": [["4514", "Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es"], ["4515", "Montijo (Terminal Rodovi\u00e1rio) - Peg\u00f5es"]],
    "419": [["4502", "Alcochete - Passil"]],
    "426": [["4516", "Montijo (Terminal Rodovi\u00e1rio) - Rio Frio"]],
    "431": [["4703", "Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio), via Alcochete e Samouco"]],
    "432": [["4704", "Atalaia - Lisboa (Oriente)"]],
    "435": [["4705", "Lisboa (Oriente) - Samouco"], ["4707", "Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio)"]],
    "440": [["4643", "Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)"]],
    "441": [["4210", "Canha - Foros Boavista"], ["4212", "Foros Boavista - Peg\u00f5es"]],
    "444": [["4520", "Faias - Peg\u00f5es"]],
    "446": [["4521", "Faias - Pinhal Novo"], ["4522", "Faias - Poceir\u00e3o"]],
    "447": [["4524", "Palmela (Terminal) - Peg\u00f5es"]],
    "451": [["4530", "Bairro Vila Morena - Pinhal Novo"]],
    "453": [["4201", "Afonsoeiro - Bairro da Liberdade"], ["4706", "S\u00e3o Francisco - Lisboa (Oriente)"]],
    "8080": [["4905", "Faias - Vendas Novas"]],
    "8902": [["4902", "Landeira - Peg\u00f5es"], ["4906", "Set\u00fabal (ITS) - Vendas Novas, via Landeira"]],
    "Nova": [["4208", "Montijo (Terminal Rodovi\u00e1rio) - Sarilhos Grandes (Estr. 4 Marcos)"], ["4211", "Craveiras - Peg\u00f5es | Circular"], ["4503", "Atalaia - Jardia"]]
  },
  "Palmela": {
    "257": [["4560", "Cabanas - Vila Nogueira de Azeit\u00e3o"], ["4561", "Cabanas - Vila Nogueira de Azeit\u00e3o, via Quinta do Pic\u00e3o"]],
    "305": [["4610", "Bairro dos Marinheiros - Barreiro (Terminal)"]],
    "311 (Adaptado)": [["4611", "Penalva - Moita (Esc. Secund\u00e1ria)"]],
    "313 (Adaptado)": [["4532", "Moita - Quatro Marcos"]],
    "326 (Adaptado)": [["4531", "Moita - Palmela (Terminal)"]],
    "413": [["4512", "Alcochete (Freeport) - Set\u00fabal (ITS), via Alto Estanqueiro"], ["4513", "Alcochete (Freeport) - Pinhal Novo"], ["4517", "Montijo (Terminal Rodovi\u00e1rio) - Set\u00fabal (ITS)"], ["4523", "Montijo (Terminal Rodovi\u00e1rio) - Pinhal Novo"]],
    "414": [["4514", "Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es"]],
    "416": [["4514", "Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es"], ["4515", "Montijo (Terminal Rodovi\u00e1rio) - Peg\u00f5es"]],
    "426": [["4516", "Montijo (Terminal Rodovi\u00e1rio) - Rio Frio"]],
    "444": [["4520", "Faias - Peg\u00f5es"]],
    "446": [["4521", "Faias - Pinhal Novo"], ["4522", "Faias - Poceir\u00e3o"]],
    "447": [["4305", "Brejos do Assa - Palmela (Terminal)"], ["4524", "Palmela (Terminal) - Peg\u00f5es"]],
    "448": [["4322", "Pinhal Novo - Rio Frio"]],
    "449": [["4320", "Pinhal Novo | Circular"], ["4321", "Pinhal Novo - Qta do Anjo"]],
    "451": [["4530", "Bairro Vila Morena - Pinhal Novo"]],
    "454": [["4302", "Palmela (Esta\u00e7\u00e3o) - Palmela (Terminal)"], ["4307", "Loja Nova - Palmela (Terminal)"]],
    "455": [["4311", "Asseiceira - Poceir\u00e3o"]],
    "563": [["4715", "Lisboa (Oriente) - Set\u00fabal (ITS), via Pinhal Novo"]],
    "565": [["4710", "Lisboa (Oriente) - Palmela (Terminal)"], ["4711", "Lisboa (Oriente) - Pinhal Novo"]],
    "604": [["4551", "Palmela (USF) - Set\u00fabal (Av. Lu\u00edsa Todi)"]],
    "610": [["4541", "Algeruz - Set\u00fabal (Av. Lu\u00edsa Todi)"]],
    "680": [["4301", "Palmela (Centro) - Palmela (Terminal)"]],
    "708": [["4545", "Biscainho - Set\u00fabal (Bela Vista)"]],
    "709": [["4901", "Landeira - Set\u00fabal (ITS)"]],
    "710": [["4310", "\u00c1guas de Moura - Poceir\u00e3o"]],
    "711": [["4312", "Poceir\u00e3o - Vale Abrunheira (X), via Fernando P\u00f3"]],
    "758": [["4548", "Lagame\u00e7as - Set\u00fabal (ITS)"]],
    "764": [["4540", "\u00c1guas de Moura - Set\u00fabal (ITS)"], ["4544", "Bairro Marga\u00e7a - Set\u00fabal (ITS)"]],
    "765": [["4540", "\u00c1guas de Moura - Set\u00fabal (ITS)"], ["4544", "Bairro Marga\u00e7a - Set\u00fabal (ITS)"]],
    "767": [["4547", "Cabanas - Set\u00fabal (ITS)"], ["4549", "Palmela (Terminal) - Set\u00fabal (ITS)"], ["4562", "Set\u00fabal (ITS) - Vila Nogueira de Azeit\u00e3o, via Palmela (Esta\u00e7\u00e3o)"]],
    "768": [["4306", "Cabanas - Palmela (Terminal)"], ["4550", "Palmela (Terminal) - Vila Nogueira de Azeit\u00e3o"], ["4562", "Set\u00fabal (ITS) - Vila Nogueira de Azeit\u00e3o, via Palmela (Esta\u00e7\u00e3o)"]],
    "774": [["4546", "Biscainho - Set\u00fabal (ITS)"]],
    "779": [["4542", "Algeruz - Set\u00fabal (ITS)"], ["4543", "Algeruz - Set\u00fabal (ITS), via Po\u00e7oilos"]],
    "8080": [["4905", "Faias - Vendas Novas"]],
    "8902": [["4902", "Landeira - Peg\u00f5es"], ["4906", "Set\u00fabal (ITS) - Vendas Novas, via Landeira"]],
    "Nova": [["3630", "Brevemente | Azeit\u00e3o - Penalva (Esta\u00e7\u00e3o)"], ["4303", "Palmela | Circular"], ["4304", "Palmela (Terminal) - Penalva"], ["4308", "Palmela (Terminal) - Pinhal Novo (Esta\u00e7\u00e3o)"], ["4313", "Cabanas - Penalva"], ["4612", "Bairro dos Marinheiros - Palmela (Terminal)"]]
  },
  "Seixal": {
    "108": [["3103", "Corroios (Esta\u00e7\u00e3o) - Paio Pires (Farinheiras)"]],
    "110": [["3513", "Cacilhas (Terminal) - Santa Marta do Pinhal"]],
    "112": [["3114", "Foros de Amora - Paio Pires (Quinta Flam\u00e2ncia)"]],
    "114": [["3508", "Cacilhas (Terminal) - Paio Pires (Centro)"]],
    "116": [["3102", "Brevemente | Aroeira - Paio Pires (Quinta Flam\u00e2ncia)"], ["3122", "Brevemente | Verdizela - Cruz de Pau"], ["3521", "Cruz de Pau - Fonta da Telha"], ["3523", "Fonte da Telha - Paio Pires (Quinta Flam\u00e2ncia), via Seixal (Terminal Fluvial) e Foros de Amora (Esta\u00e7\u00e3o)"]],
    "120": [["3515", "Caparica (Pilotos) -  Corroios"]],
    "121": [["3526", "Laranjeiro - Pinheirinho"]],
    "126 (Adaptado)": [["3507", "Cacilhas (Terminal) - Marisol"], ["3524", "Hospital Garcia de Orta - Marisol"]],
    "137": [["3110", "Fogueteiro (Esta\u00e7\u00e3o) - Redondos"], ["3120", "Redondos - Seixal (Terminal Fluvial)"]],
    "139": [["3519", "Costa da Caparica (Terminal) - Corroios (Esta\u00e7\u00e3o)"]],
    "143": [["3518", "Corroios (Esta\u00e7\u00e3o) - Vale de Figueira"]],
    "149": [["3512", "Cacilhas (Terminal) - Quinta Princesa"]],
    "159": [["3716", "Lisboa (Sete Rios) - Marisol"]],
    "159 (adaptada)": [["3501", "Brevemente | Almada Forum - Marisol, via Sobreda"]],
    "162": [["3717", "Lisboa (Sete Rios) - Quinta do Brasileiro"]],
    "163": [["3520", "Costa da Caparica (Terminal) - Quinta do Brasileiro"]],
    "169": [["3715", "Lisboa (M. Pombal) - Santa Marta do Pinhal"]],
    "172": [["3522", "Fonte da Telha - Paio Pires (Centro)"]],
    "175 (adaptada)": [["3501", "Brevemente | Almada Forum - Marisol, via Sobreda"]],
    "181 (adaptada)": [["3525", "Hospital Garcia de Orta - Miratejo"]],
    "184": [["3111", "Fogueteiro (Esta\u00e7\u00e3o) - Seixal (Terminal Fluvial)"]],
    "191": [["3514", "Cacilhas (Terminal) - Vale de Milha\u00e7os"], ["3104", "Corroios (Esta\u00e7\u00e3o) - Vale de Milha\u00e7os"]],
    "192": [["3511", "Cacilhas (Terminal) - Pinheirinho"]],
    "195": [["3119", "Pinhal Conde Cunha - Seixal (Terminal Fluvial)"]],
    "196": [["3510", "Cacilhas (Terminal) - Pilotos"]],
    "197": [["3504", "Brevemente | Bairro Fundo Fomento - Quintinha"]],
    "198": [["3502", "Almada Forum - Paio Pires (Centro)"], ["3527", "Monte da Caparica (FCT) - Paio Pires (Bairro Cucena)"], ["3528", "Monte da Caparica (FCT) - Paio Pires (Centro)"]],
    "199": [["3509", "Cacilhas (Terminal) - Paio Pires (Centro), via Seixal (Terminal Fluvial) e Amora"]],
    "1A": [["3101", "Amora - Foros de Amora (Esta\u00e7\u00e3o)"]],
    "1C": [["3516", "Charneca da Caparica - Corroios (Esta\u00e7\u00e3o)"]],
    "1F": [["3112", "Fogueteiro (Esta\u00e7\u00e3o) - Seixal (Terminal Fluvial), via Paio Pires"]],
    "1N": [["3620", "Coina (Esta\u00e7\u00e3o) - Quinta do Conde"]],
    "203": [["3536", "Cacilhas (Terminal) - Sesimbra (Terminal)"]],
    "207": [["3721", "Lisboa (Sete Rios) - Sesimbra (Terminal)"]],
    "208": [["3635", "Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal), via Azeit\u00e3o e Sampaio"]],
    "211": [["3549", "Quinta do Conde - Sesimbra (Terminal), via Sampaio e Marco do Grilo"]],
    "219": [["3548", "Marco do Grilo - Quinta do Conde"]],
    "236": [["3107", "Laranjeiras - Marco do Grilo"]],
    "252": [["3720", "Lisboa (Sete Rios) - Quinta do Conde"]],
    "254": [["3535", "Cacilhas (Terminal) - Quinta do Conde"], ["3546", "Cruz de Pau - Quinta do Conde"], ["3547", "Fogueteiro (Esta\u00e7\u00e3o) - Quinta do Conde"]],
    "260": [["3545", "Corroios (Esta\u00e7\u00e3o) - Sesimbra (Terminal)"]],
    "2A": [["3108", "Fogueteiro - Foros de Amora (Esta\u00e7\u00e3o)"]],
    "2C (Adaptado)": [["3506", "Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o), via Miratejo"]],
    "2F": [["3113", "Fogueteiro (Esta\u00e7\u00e3o) - Seixal (Terminal Fluvial), via Quinta do Cabral"]],
    "2N": [["3626", "Coina (Esta\u00e7\u00e3o) - Vila Fresca de Azeit\u00e3o"]],
    "2ND": [["3543", "Coina (Esta\u00e7\u00e3o) - Quinta do Conde, via Estrada de Coina"]],
    "302": [["4620", "Moita - Paio Pires"]],
    "3C": [["3517", "Chegadinho - Corroios (Esta\u00e7\u00e3o)"]],
    "3F": [["3105", "Fern\u00e3o Ferro - Fogueteiro (Esta\u00e7\u00e3o)"]],
    "4F": [["3109", "Fogueteiro (Esta\u00e7\u00e3o) - Parque Empresarial do Seixal"]],
    "583": [["3610", "Brevemente | Cacilhas (Terminal) - Set\u00fabal (ITS), via A2"]],
    "754": [["4631", "Fogueteiro (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)"]],
    "755": [["4630", "Corroios (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)"]],
    "783": [["3605", "Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o"]],
    "Nova": [["3106", "Brevemente | Coina (Esta\u00e7\u00e3o) - Fern\u00e3o Ferro"], ["3115", "Brevemente | Marisol - Foros de Amora (Esta\u00e7\u00e3o), via Corroios (Esta\u00e7\u00e3o)"], ["3116", "Brevemente | Marisol - Seixal (Terminal Fluvial), via Corroios (Esta\u00e7\u00e3o)"], ["3117", "Brevemente | Marisol (Valadares) - Foros de Amora (Esta\u00e7\u00e3o)"], ["3118", "Brevemente | Marisol (Valadares) - Seixal (Terminal Fluvial)"], ["3121", "Seixal | Circular"], ["3503", "Brevemente | Almada Forum - Vale de Milha\u00e7os"], ["3505", "Brevemente | Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o)"], ["3540", "Brevemente | Alfarim - Coina (Esta\u00e7\u00e3o)"], ["3541", "Brevemente | Coina (Esta\u00e7\u00e3o) - Fern\u00e3o Ferro, via Casal do Sapo e Pinhal do General"], ["3542", "Brevemente | Coina (Esta\u00e7\u00e3o) - Praia do Meco"], ["3544", "Brevemente | Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal)"], ["3601", "Barreiro - Cova da Piedade (Metro)"], ["3615", "Barreiro - Seixal"], ["3625", "Brevemente | Barreiro - Sesimbra (Terminal)"], ["4621", "Moita - Seixal (Terminal Fluvial)"]]
  },
  "Sesimbra": {
    "1N": [["3620", "Coina (Esta\u00e7\u00e3o) - Quinta do Conde"]],
    "201": [["3203", "Azoia - Sesimbra (Terminal)"], ["3204", "Azoia - Sesimbra (Terminal), via Serra da Azoia"], ["3205", "Cabo Espichel - Sesimbra (Terminal)"]],
    "203": [["3536", "Cacilhas (Terminal) - Sesimbra (Terminal)"]],
    "204": [["3206", "Carrasqueira - Sesimbra (Terminal)"]],
    "205": [["3205", "Cabo Espichel - Sesimbra (Terminal)"]],
    "207": [["3721", "Lisboa (Sete Rios) - Sesimbra (Terminal)"]],
    "208": [["3635", "Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal), via Azeit\u00e3o e Sampaio"], ["3641", "Quinta do Conde - Sampaio, via Azeit\u00e3o"]],
    "210": [["3217", "Azoia - Sampaio"]],
    "211": [["3549", "Quinta do Conde - Sesimbra (Terminal), via Sampaio e Marco do Grilo"]],
    "219": [["3548", "Marco do Grilo - Quinta do Conde"]],
    "221": [["3220", "Sesimbra | Circular"]],
    "222": [["3201", "Aldeia do Meco - Sesimbra (Terminal), via Aiana"], ["3202", "Alfarim - Sesimbra (Terminal), via Aiana"], ["3209", "Fornos - Sesimbra (Terminal), via Aiana"]],
    "223": [["3208", "Fetais - Sesimbra (Terminal)"]],
    "225": [["3640", "Azoia - Vila Nogueira de Azeit\u00e3o"]],
    "227": [["3212", "Ma\u00e7\u00e3 (Rua Macieira) - Sesimbra (Terminal)"], ["3216", "Alto das Vinhas - Sampaio"]],
    "228": [["3218", "Sesimbra (Porto de Abrigo) - Sesimbra (Terminal)"]],
    "229": [["3213", "Pinhal de Cima - Sesimbra (Terminal) | Circular"]],
    "230": [["4642", "Sesimbra (Terminal) - Set\u00fabal (Hospital)"]],
    "231": [["3201", "Aldeia do Meco - Sesimbra (Terminal), via Aiana"]],
    "234": [["3215", "Fornos - Sampaio"]],
    "240": [["3210", "Lagoa de Albufeira - Sesimbra (Terminal)"], ["3211", "Lagoa de Albufeira - Sesimbra (Terminal), via Sampaio"]],
    "243": [["3207", "Carrasqueira - Sesimbra (Terminal), via Valbom e Sampaio"], ["3221", "Valbom - Sesimbra (Terminal), via Sampaio"]],
    "245": [["3650", "Brevemente | Moita - Sesimbra (Terminal)"]],
    "247": [["3214", "Sampaio - Santana"]],
    "252": [["3720", "Lisboa (Sete Rios) - Quinta do Conde"]],
    "254": [["3535", "Cacilhas (Terminal) - Quinta do Conde"], ["3546", "Cruz de Pau - Quinta do Conde"], ["3547", "Fogueteiro (Esta\u00e7\u00e3o) - Quinta do Conde"]],
    "259": [["4640", "Casais da Serra - Vila Nogueira de Azeit\u00e3o"]],
    "260": [["3545", "Corroios (Esta\u00e7\u00e3o) - Sesimbra (Terminal)"]],
    "2N": [["3626", "Coina (Esta\u00e7\u00e3o) - Vila Fresca de Azeit\u00e3o"]],
    "2ND": [["3543", "Coina (Esta\u00e7\u00e3o) - Quinta do Conde, via Estrada de Coina"]],
    "440": [["4643", "Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)"]],
    "754": [["4631", "Fogueteiro (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)"]],
    "755": [["4630", "Corroios (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)"]],
    "770": [["4641", "Quinta do Conde - Set\u00fabal (ITS)"]],
    "783": [["3605", "Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o"]],
    "Nova": [["3219", "Brevemente | Sesimbra (R. Palames) - Sesimbra (Terminal)"], ["3222", "Quinta do Conde | Circular"], ["3540", "Brevemente | Alfarim - Coina (Esta\u00e7\u00e3o)"], ["3541", "Brevemente | Coina (Esta\u00e7\u00e3o) - Fern\u00e3o Ferro, via Casal do Sapo e Pinhal do General"], ["3542", "Brevemente | Coina (Esta\u00e7\u00e3o) - Praia do Meco"], ["3544", "Brevemente | Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal)"], ["3625", "Brevemente | Barreiro - Sesimbra (Terminal)"], ["3630", "Brevemente | Azeit\u00e3o - Penalva (Esta\u00e7\u00e3o)"]]
  },
  "Set\u00fabal": {
    "208": [["3635", "Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal), via Azeit\u00e3o e Sampaio"], ["3641", "Quinta do Conde - Sampaio, via Azeit\u00e3o"]],
    "225": [["3640", "Azoia - Vila Nogueira de Azeit\u00e3o"]],
    "230": [["4642", "Sesimbra (Terminal) - Set\u00fabal (Hospital)"]],
    "257": [["4560", "Cabanas - Vila Nogueira de Azeit\u00e3o"], ["4561", "Cabanas - Vila Nogueira de Azeit\u00e3o, via Quinta do Pic\u00e3o"]],
    "259": [["4640", "Casais da Serra - Vila Nogueira de Azeit\u00e3o"]],
    "2N": [["3626", "Coina (Esta\u00e7\u00e3o) - Vila Fresca de Azeit\u00e3o"]],
    "413": [["4512", "Alcochete (Freeport) - Set\u00fabal (ITS), via Alto Estanqueiro"], ["4517", "Montijo (Terminal Rodovi\u00e1rio) - Set\u00fabal (ITS)"]],
    "440": [["4643", "Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)"]],
    "561": [["4725", "Lisboa (Sete Rios) - Set\u00fabal (ITS)"]],
    "562": [["4720", "Lisboa (Oriente) - Set\u00fabal (ITS)"]],
    "563": [["4715", "Lisboa (Oriente) - Set\u00fabal (ITS), via Pinhal Novo"]],
    "583": [["3610", "Brevemente | Cacilhas (Terminal) - Set\u00fabal (ITS), via A2"]],
    "601": [["4406", "Manteigadas - Set\u00fabal (Mercado)"], ["4408", "Manteigadas - Set\u00fabal (Mercado), via Bela Vista"], ["4412", "Morgada - Set\u00fabal (Mercado)"], ["4413", "Morgada - Set\u00fabal (Mercado), via Bela Vista"], ["4436", "Set\u00fabal (Mercado) - Set\u00fabal (Av. Soeiro Pereira Gomes)"]],
    "602": [["4416", "Po\u00e7o Mouro - Set\u00fabal (ITS)"], ["4417", "Po\u00e7o Mouro - Set\u00fabal (ITS), via Manteigadas"], ["4418", "Set\u00fabal (Alegro) - Set\u00fabal (Av. 5 Outubro)"], ["4420", "Set\u00fabal (Alegro) - Set\u00fabal (ITS)"]],
    "604": [["4403", "Fonte da Talha - Set\u00fabal (Av. Lu\u00edsa Todi)"], ["4423", "Amoreiras \u2013 Set\u00fabal (Av. Lu\u00edsa Todi)"], ["4551", "Palmela (USF) - Set\u00fabal (Av. Lu\u00edsa Todi)"]],
    "605": [["4431", "Set\u00fabal (ITS) - Set\u00fabal (Quinta Varzinha)"]],
    "607": [["4434", "Set\u00fabal (Mercado 2 de Abril) - Set\u00fabal (R. Timor)"]],
    "608": [["4424", "Set\u00fabal (Bairro Viso) - Manteigadas"], ["4425", "Set\u00fabal (Escola Viso) - Mitrena"], ["4426", "Set\u00fabal (Bairro Viso) - Set\u00fabal (CHEset\u00fabal)"]],
    "609": [["4421", "Set\u00fabal (Bairro Camolas) - Set\u00fabal (Casal Figueiras)"], ["4422", "Set\u00fabal (Bairro Camolas) - Set\u00fabal (Casal Figueiras), via Bairro do Viso"], ["4428", "Set\u00fabal (Casal Figueiras) - Vale Ana Gomes"]],
    "610": [["4438", "Set\u00fabal (Monte Belo Norte) - Set\u00fabal (Saboaria)"], ["4440", "Set\u00fabal (Monte Belo Norte) - Set\u00fabal (Saboaria), via Alegro"], ["4541", "Algeruz - Set\u00fabal (Av. Lu\u00edsa Todi)"]],
    "612": [["4419", "Brejos Canes - Set\u00fabal (Saboaria)"], ["4441", "Set\u00fabal (Saboaria) - Set\u00fabal (Vale Cobro)"]],
    "614": [["4421", "Set\u00fabal (Bairro Camolas) - Set\u00fabal (Casal Figueiras)"], ["4422", "Set\u00fabal (Bairro Camolas) - Set\u00fabal (Casal Figueiras), via Bairro do Viso"], ["4433", "Alto Guerra - Set\u00fabal (Casal Figueiras)"]],
    "616": [["4407", "Manteigadas - Set\u00fabal (Mercado), via Bairro da Carmona"]],
    "708": [["4545", "Biscainho - Set\u00fabal (Bela Vista)"]],
    "709": [["4901", "Landeira - Set\u00fabal (ITS)"]],
    "723": [["4472", "Praia do Creiro - Set\u00fabal (ITS)"], ["4476", "Praias Arr\u00e1bida | Circular"]],
    "725": [["4474", "Figueirinha - Set\u00fabal (Alegro)"]],
    "726": [["4471", "Praia Albarquel | Circular"]],
    "727": [["4470", "Brejos Azeit\u00e3o - Praia do Creiro"]],
    "751": [["4414", "Out\u00e3o (Hospital) - Set\u00fabal (ITS)"], ["4415", "Out\u00e3o (Hospital) - Set\u00fabal (ITS), via vale da Rasca"]],
    "754": [["4631", "Fogueteiro (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)"]],
    "755": [["4630", "Corroios (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)"]],
    "756": [["4452", "Mitrena (Portucel) - Set\u00fabal (ITS)"]],
    "757": [["4453", "Mitrena (Portucel) - Set\u00fabal (ITS), via Estrada Gra\u00e7a"]],
    "758": [["4548", "Lagame\u00e7as - Set\u00fabal (ITS)"]],
    "764": [["4540", "\u00c1guas de Moura - Set\u00fabal (ITS)"], ["4544", "Bairro Marga\u00e7a - Set\u00fabal (ITS)"]],
    "765": [["4540", "\u00c1guas de Moura - Set\u00fabal (ITS)"], ["4544", "Bairro Marga\u00e7a - Set\u00fabal (ITS)"]],
    "766": [["4442", "Praias do Sado (Esta\u00e7\u00e3o) - Set\u00fabal (Bela Vista)"]],
    "767": [["4547", "Cabanas - Set\u00fabal (ITS)"], ["4549", "Palmela (Terminal) - Set\u00fabal (ITS)"], ["4562", "Set\u00fabal (ITS) - Vila Nogueira de Azeit\u00e3o, via Palmela (Esta\u00e7\u00e3o)"]],
    "768": [["4550", "Palmela (Terminal) - Vila Nogueira de Azeit\u00e3o"], ["4562", "Set\u00fabal (ITS) - Vila Nogueira de Azeit\u00e3o, via Palmela (Esta\u00e7\u00e3o)"]],
    "770": [["4641", "Quinta do Conde - Set\u00fabal (ITS)"]],
    "774": [["4546", "Biscainho - Set\u00fabal (ITS)"]],
    "776": [["4451", "Mitrena (Lisnave) - Set\u00fabal (ITS)"]],
    "779": [["4542", "Algeruz - Set\u00fabal (ITS)"], ["4543", "Algeruz - Set\u00fabal (ITS), via Po\u00e7oilos"]],
    "780": [["4402", "Estefanilha - Set\u00fabal (ITS)"], ["4437", "Faralh\u00e3o - Set\u00fabal (ITS)"]],
    "781": [["4411", "Morgada - Set\u00fabal (ITS)"]],
    "783": [["3605", "Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o"]],
    "797": [["4439", "Praias do Sado - Set\u00fabal (ITS)"]],
    "8902": [["4906", "Set\u00fabal (ITS) - Vendas Novas, via Landeira"]],
    "Nova": [["3625", "Brevemente | Barreiro - Sesimbra (Terminal)"], ["3630", "Brevemente | Azeit\u00e3o - Penalva (Esta\u00e7\u00e3o)"], ["4401", "Cachofarra - Set\u00fabal (Hospital)"], ["4404", "Interfaces Set\u00fabal | Circular"], ["4405", "Livramento-Montebelo | Circular"], ["4409", "Manteigadas - Viso"], ["4410", "Manteigadas (Esc. Profissional) - Set\u00fabal (Alegro)"], ["4427", "Set\u00fabal (Bela Vista) - Set\u00fabal (Mercado)"], ["4429", "Set\u00fabal (Centro Sa\u00fade) - Set\u00fabal (Mercado)"], ["4430", "Set\u00fabal (Hospital) - Set\u00fabal (Montalv\u00e3o)"], ["4432", "Set\u00fabal (ITS) - Vale de Choupo"], ["4435", "Biscainho - Faralh\u00e3o"], ["4443", "Set\u00fabal (Polit\u00e9cnico) - Praias do Sado"], ["4460", "Azeit\u00e3o | Circular"], ["4475", "Portinho da Arr\u00e1bida - Viso"]]
  },
  "Vendas Novas (CIMAC)": {
    "709": [["4901", "Landeira - Set\u00fabal (ITS)"]],
    "8080": [["4905", "Faias - Vendas Novas"]],
    "8902": [["4902", "Landeira - Peg\u00f5es"], ["4906", "Set\u00fabal (ITS) - Vendas Novas, via Landeira"]]
  }
};
const diretorio_operadores = {
  "CM Almada": {
    "Flexibus": [["3005", "Brevemente | Flexibus Almada | Circular"]]
  },
  "Nova": {
    "Nova": [["3002", "Brevemente | Almada (Parque Urbano) - Pragal (Esta\u00e7\u00e3o)"], ["3006", "Brevemente | Aroeira | Circular"], ["3009", "Cacilhas (terminal - Trafaria (Terminal)"], ["3016", "Brevemente | Charneca da Caparica - Lazarim"], ["3017", "Charneca da Caparica - Pragal (Esta\u00e7\u00e3o)"], ["3019", "Charneca da Caparica - Trafaria (Terminal)"], ["3020", "Brevemente | Charneca da Caparica | Circular"], ["3021", "Costa da Caparica - Monte da Caparica (FCT)"], ["3025", "Brevemente | Costa da Caparica (Terminal) - Pragal (Esta\u00e7\u00e3o), via IC20"], ["3028", "Brevemente | Lazarim | Circular"], ["3029", "Brevemente | Marco Caba\u00e7o | Circular"], ["3031", "Brevemente | Monte da Caparica - Quintinha"], ["3033", "Brevemente | Monte da Caparica | Circular"], ["3035", "Pragal (Esta\u00e7\u00e3o) - Quinta do Texugo"], ["3036", "Pragal (Esta\u00e7\u00e3o) - Vale Flores"], ["3037", "Brevemente | Quintinha | Circular"], ["3106", "Brevemente | Coina (Esta\u00e7\u00e3o) - Fern\u00e3o Ferro"], ["3115", "Brevemente | Marisol - Foros de Amora (Esta\u00e7\u00e3o), via Corroios (Esta\u00e7\u00e3o)"], ["3116", "Brevemente | Marisol - Seixal (Terminal Fluvial), via Corroios (Esta\u00e7\u00e3o)"], ["3117", "Brevemente | Marisol (Valadares) - Foros de Amora (Esta\u00e7\u00e3o)"], ["3118", "Brevemente | Marisol (Valadares) - Seixal (Terminal Fluvial)"], ["3121", "Seixal | Circular"], ["3219", "Brevemente | Sesimbra (R. Palames) - Sesimbra (Terminal)"], ["3222", "Quinta do Conde | Circular"], ["3503", "Brevemente | Almada Forum - Vale de Milha\u00e7os"], ["3505", "Brevemente | Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o)"], ["3540", "Brevemente | Alfarim - Coina (Esta\u00e7\u00e3o)"], ["3541", "Brevemente | Coina (Esta\u00e7\u00e3o) - Fern\u00e3o Ferro, via Casal do Sapo e Pinhal do General"], ["3542", "Brevemente | Coina (Esta\u00e7\u00e3o) - Praia do Meco"], ["3544", "Brevemente | Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal)"], ["3601", "Barreiro - Cova da Piedade (Metro)"], ["3615", "Barreiro - Seixal"], ["3625", "Brevemente | Barreiro - Sesimbra (Terminal)"], ["3630", "Brevemente | Azeit\u00e3o - Penalva (Esta\u00e7\u00e3o)"], ["3701", "Brevemente | Almada (Centro Sul) - Alg\u00e9s (Terminal)"], ["3706", "Brevemente | Charneca da Caparica - Lisboa (Sete Rios), via Feij\u00f3"], ["3708", "Brevemente | Costa da Caparica (Terminal) - Lisboa (C. Sodr\u00e9)"], ["4001", "Alcochete | Circular"], ["4002", "S\u00e3o Francisco | Circular"], ["4208", "Montijo (Terminal Rodovi\u00e1rio) - Sarilhos Grandes (Estr. 4 Marcos)"], ["4211", "Craveiras - Peg\u00f5es | Circular"], ["4303", "Palmela | Circular"], ["4304", "Palmela (Terminal) - Penalva"], ["4308", "Palmela (Terminal) - Pinhal Novo (Esta\u00e7\u00e3o)"], ["4313", "Cabanas - Penalva"], ["4401", "Cachofarra - Set\u00fabal (Hospital)"], ["4404", "Interfaces Set\u00fabal | Circular"], ["4405", "Livramento-Montebelo | Circular"], ["4409", "Manteigadas - Viso"], ["4410", "Manteigadas (Esc. Profissional) - Set\u00fabal (Alegro)"], ["4427", "Set\u00fabal (Bela Vista) - Set\u00fabal (Mercado)"], ["4429", "Set\u00fabal (Centro Sa\u00fade) - Set\u00fabal (Mercado)"], ["4430", "Set\u00fabal (Hospital) - Set\u00fabal (Montalv\u00e3o)"], ["4432", "Set\u00fabal (ITS) - Vale de Choupo"], ["4435", "Biscainho - Faralh\u00e3o"], ["4443", "Set\u00fabal (Polit\u00e9cnico) - Praias do Sado"], ["4460", "Azeit\u00e3o | Circular"], ["4475", "Portinho da Arr\u00e1bida - Viso"], ["4503", "Atalaia - Jardia"], ["4612", "Bairro dos Marinheiros - Palmela (Terminal)"], ["4621", "Moita - Seixal (Terminal Fluvial)"]]
  },
  "Rodovi\u00e1ria do Alentejo": {
    "8080": [["4905", "Faias - Vendas Novas"]],
    "8902": [["4902", "Landeira - Peg\u00f5es"], ["4906", "Set\u00fabal (ITS) - Vendas Novas, via Landeira"]]
  },
  "SulFertagus": {
    "1A": [["3101", "Amora - Foros de Amora (Esta\u00e7\u00e3o)"]],
    "1C": [["3516", "Charneca da Caparica - Corroios (Esta\u00e7\u00e3o)"]],
    "1F": [["3112", "Fogueteiro (Esta\u00e7\u00e3o) - Seixal (Terminal Fluvial), via Paio Pires"]],
    "1N": [["3620", "Coina (Esta\u00e7\u00e3o) - Quinta do Conde"]],
    "1P": [["3027", "Hospital Garcia de Orta - Sobreda"]],
    "2A": [["3108", "Fogueteiro - Foros de Amora (Esta\u00e7\u00e3o)"]],
    "2C (Adaptado)": [["3506", "Cacilhas (Terminal) - Corroios (Esta\u00e7\u00e3o), via Miratejo"]],
    "2F": [["3113", "Fogueteiro (Esta\u00e7\u00e3o) - Seixal (Terminal Fluvial), via Quinta do Cabral"]],
    "2N": [["3626", "Coina (Esta\u00e7\u00e3o) - Vila Fresca de Azeit\u00e3o"]],
    "2ND": [["3543", "Coina (Esta\u00e7\u00e3o) - Quinta do Conde, via Estrada de Coina"]],
    "3C": [["3517", "Chegadinho - Corroios (Esta\u00e7\u00e3o)"]],
    "3F": [["3105", "Fern\u00e3o Ferro - Fogueteiro (Esta\u00e7\u00e3o)"]],
    "4F": [["3109", "Fogueteiro (Esta\u00e7\u00e3o) - Parque Empresarial do Seixal"]]
  },
  "Transportes Sul do Tejo": {
    "101": [["3001", "Almada (Cristo Rei) - Cacilhas (Terminal)"]],
    "102": [["3003", "Almada Forum - Cacilhas (Terminal)"]],
    "106": [["3007", "Bairro Fundo Fomento - Cacilhas (Terminal)"]],
    "108": [["3103", "Corroios (Esta\u00e7\u00e3o) - Paio Pires (Farinheiras)"]],
    "110": [["3513", "Cacilhas (Terminal) - Santa Marta do Pinhal"]],
    "112": [["3114", "Foros de Amora - Paio Pires (Quinta Flam\u00e2ncia)"]],
    "114": [["3508", "Cacilhas (Terminal) - Paio Pires (Centro)"]],
    "116": [["3102", "Brevemente | Aroeira - Paio Pires (Quinta Flam\u00e2ncia)"], ["3122", "Brevemente | Verdizela - Cruz de Pau"], ["3521", "Cruz de Pau - Fonta da Telha"], ["3523", "Fonte da Telha - Paio Pires (Quinta Flam\u00e2ncia), via Seixal (Terminal Fluvial) e Foros de Amora (Esta\u00e7\u00e3o)"]],
    "117": [["3014", "Cacilhas (terminal) - Raposeira"]],
    "120": [["3515", "Caparica (Pilotos) -  Corroios"]],
    "121": [["3526", "Laranjeiro - Pinheirinho"]],
    "123 (Adaptado)": [["3013", "Cacilhas (Terminal) - Monte da Caparica"]],
    "124 (Adaptado)": [["3022", "Costa da Caparica (Terminal) - Hospital Garcia de Orta"]],
    "126 (Adaptado)": [["3507", "Cacilhas (Terminal) - Marisol"], ["3524", "Hospital Garcia de Orta - Marisol"]],
    "127 (Adaptado)": [["3012", "Cacilhas (Terminal) - Fonte da Telha"]],
    "129 (Adaptado)": [["3030", "Fonte da Telha - Monte da Caparica (FCT)"]],
    "130 (Adaptado)": [["3030", "Fonte da Telha - Monte da Caparica (FCT)"]],
    "135": [["3011", "Cacilhas (Terminal) - Costa da Caparica"]],
    "137": [["3110", "Fogueteiro (Esta\u00e7\u00e3o) - Redondos"], ["3120", "Redondos - Seixal (Terminal Fluvial)"]],
    "139": [["3519", "Costa da Caparica (Terminal) - Corroios (Esta\u00e7\u00e3o)"]],
    "143": [["3518", "Corroios (Esta\u00e7\u00e3o) - Vale de Figueira"]],
    "145 (Adaptado)": [["3010", "Cacilhas (Terminal) - Charneca da Caparica"]],
    "146": [["3008", "Ban\u00e1tica - Quintinha"]],
    "146 (Adaptado)": [["3032", "Brevemente | Monte da Caparica (FCT) - Quinta do Texugo"], ["3034", "Porto Brand\u00e3o (Terminal) - Quinta do Texugo"]],
    "149": [["3512", "Cacilhas (Terminal) - Quinta Princesa"]],
    "151": [["3704", "Charneca da Caparica - Lisboa (M. Pombal)"]],
    "153": [["3710", "Costa da Caparica (Terminal) - Lisboa (Sete Rios)"]],
    "155": [["3709", "Costa da Caparica (Terminal) - Lisboa (M. Pombal)"]],
    "158": [["3711", "Monte da Caparica (FCT) - Lisboa (Sete Rios)"]],
    "159": [["3707", "Charneca da Caparica - Lisboa (Sete Rios), via Sobreda"], ["3716", "Lisboa (Sete Rios) - Marisol"]],
    "159 (adaptada)": [["3501", "Brevemente | Almada Forum - Marisol, via Sobreda"]],
    "160": [["3703", "Almada (Parque Urbano) - Lisboa (Sete Rios)"]],
    "161": [["3710", "Costa da Caparica (Terminal) - Lisboa (Sete Rios)"]],
    "162": [["3717", "Lisboa (Sete Rios) - Quinta do Brasileiro"]],
    "163": [["3520", "Costa da Caparica (Terminal) - Quinta do Brasileiro"]],
    "167": [["3023", "Brevemente | Costa da Caparica (terminal) - Laranjeiro"]],
    "169": [["3715", "Lisboa (M. Pombal) - Santa Marta do Pinhal"]],
    "171": [["3015", "Charneca da Caparica - Cova do Vapor"]],
    "172": [["3522", "Fonte da Telha - Paio Pires (Centro)"]],
    "174 (Adaptado)": [["3024", "Costa da Caparica (Terminal) - Pragal (Esta\u00e7\u00e3o)"]],
    "175 (adaptada)": [["3501", "Brevemente | Almada Forum - Marisol, via Sobreda"]],
    "176": [["3702", "Almada (Parque Urbano) - Lisboa (C. Universit\u00e1ria)"]],
    "179(adaptada)": [["3004", "Almada Forum - Marisol"]],
    "180": [["3018", "Brevemente | Charneca da Caparica - Sobreda"]],
    "181 (adaptada)": [["3525", "Hospital Garcia de Orta - Miratejo"]],
    "182": [["3026", "Cova da Piedade - Hospital Garcia de Orta"]],
    "184": [["3111", "Fogueteiro (Esta\u00e7\u00e3o) - Seixal (Terminal Fluvial)"]],
    "190 (adaptada)": [["3705", "Brevemente | Charneca da Caparica - Lisboa (Sete Rios)"]],
    "191": [["3514", "Cacilhas (Terminal) - Vale de Milha\u00e7os"], ["3104", "Corroios (Esta\u00e7\u00e3o) - Vale de Milha\u00e7os"]],
    "192": [["3511", "Cacilhas (Terminal) - Pinheirinho"]],
    "195": [["3119", "Pinhal Conde Cunha - Seixal (Terminal Fluvial)"]],
    "196": [["3510", "Cacilhas (Terminal) - Pilotos"]],
    "197": [["3504", "Brevemente | Bairro Fundo Fomento - Quintinha"]],
    "198": [["3502", "Almada Forum - Paio Pires (Centro)"], ["3527", "Monte da Caparica (FCT) - Paio Pires (Bairro Cucena)"], ["3528", "Monte da Caparica (FCT) - Paio Pires (Centro)"]],
    "199": [["3509", "Cacilhas (Terminal) - Paio Pires (Centro), via Seixal (Terminal Fluvial) e Amora"]],
    "201": [["3203", "Azoia - Sesimbra (Terminal)"], ["3204", "Azoia - Sesimbra (Terminal), via Serra da Azoia"], ["3205", "Cabo Espichel - Sesimbra (Terminal)"]],
    "203": [["3536", "Cacilhas (Terminal) - Sesimbra (Terminal)"]],
    "204": [["3206", "Carrasqueira - Sesimbra (Terminal)"]],
    "205": [["3205", "Cabo Espichel - Sesimbra (Terminal)"]],
    "207": [["3721", "Lisboa (Sete Rios) - Sesimbra (Terminal)"]],
    "208": [["3635", "Coina (Esta\u00e7\u00e3o) - Sesimbra (Terminal), via Azeit\u00e3o e Sampaio"], ["3641", "Quinta do Conde - Sampaio, via Azeit\u00e3o"]],
    "210": [["3217", "Azoia - Sampaio"]],
    "211": [["3549", "Quinta do Conde - Sesimbra (Terminal), via Sampaio e Marco do Grilo"]],
    "219": [["3548", "Marco do Grilo - Quinta do Conde"]],
    "221": [["3220", "Sesimbra | Circular"]],
    "222": [["3201", "Aldeia do Meco - Sesimbra (Terminal), via Aiana"], ["3202", "Alfarim - Sesimbra (Terminal), via Aiana"], ["3209", "Fornos - Sesimbra (Terminal), via Aiana"]],
    "223": [["3208", "Fetais - Sesimbra (Terminal)"]],
    "225": [["3640", "Azoia - Vila Nogueira de Azeit\u00e3o"]],
    "227": [["3212", "Ma\u00e7\u00e3 (Rua Macieira) - Sesimbra (Terminal)"], ["3216", "Alto das Vinhas - Sampaio"]],
    "228": [["3218", "Sesimbra (Porto de Abrigo) - Sesimbra (Terminal)"]],
    "229": [["3213", "Pinhal de Cima - Sesimbra (Terminal) | Circular"]],
    "230": [["4642", "Sesimbra (Terminal) - Set\u00fabal (Hospital)"]],
    "231": [["3201", "Aldeia do Meco - Sesimbra (Terminal), via Aiana"]],
    "234": [["3215", "Fornos - Sampaio"]],
    "236": [["3107", "Laranjeiras - Marco do Grilo"]],
    "240": [["3210", "Lagoa de Albufeira - Sesimbra (Terminal)"], ["3211", "Lagoa de Albufeira - Sesimbra (Terminal), via Sampaio"]],
    "243": [["3207", "Carrasqueira - Sesimbra (Terminal), via Valbom e Sampaio"], ["3221", "Valbom - Sesimbra (Terminal), via Sampaio"]],
    "245": [["3650", "Brevemente | Moita - Sesimbra (Terminal)"]],
    "247": [["3214", "Sampaio - Santana"]],
    "252": [["3720", "Lisboa (Sete Rios) - Quinta do Conde"]],
    "254": [["3535", "Cacilhas (Terminal) - Quinta do Conde"], ["3546", "Cruz de Pau - Quinta do Conde"], ["3547", "Fogueteiro (Esta\u00e7\u00e3o) - Quinta do Conde"]],
    "257": [["4560", "Cabanas - Vila Nogueira de Azeit\u00e3o"], ["4561", "Cabanas - Vila Nogueira de Azeit\u00e3o, via Quinta do Pic\u00e3o"]],
    "259": [["4640", "Casais da Serra - Vila Nogueira de Azeit\u00e3o"]],
    "260": [["3545", "Corroios (Esta\u00e7\u00e3o) - Sesimbra (Terminal)"]],
    "302": [["4620", "Moita - Paio Pires"]],
    "305": [["4610", "Bairro dos Marinheiros - Barreiro (Terminal)"]],
    "307 (Adaptado)": [["4604", "Barreiro (Terminal) - Moita (Escola Fragata do Tejo)"]],
    "311 (Adaptado)": [["4611", "Penalva - Moita (Esc. Secund\u00e1ria)"]],
    "312 (Adaptado)": [["4102", "Cabe\u00e7o Verde - Sarilhos Pequenos"]],
    "313 (Adaptado)": [["4104", "Moita | Circular"], ["4532", "Moita - Quatro Marcos"]],
    "317": [["4602", "Alhos Vedros (Esta\u00e7\u00e3o) - Barreiro (Terminal)"]],
    "318": [["4103", "Moita (Esta\u00e7\u00e3o) - Sarilhos Pequenos"]],
    "326 (Adaptado)": [["4531", "Moita - Palmela (Terminal)"]],
    "330": [["4605", "Lavradio - Pinhal do Forno"]],
    "333": [["4701", "Lisboa (Oriente) - Vale da Amoreira"]],
    "336 (Adaptado)": [["4101", "Alhos Vedros (Escola Jos\u00e9 Afonso) - Arroteias"]],
    "401": [["4204", "Bairro do Charqueir\u00e3o - Montijo (Terminal Fluvial)"], ["4205", "Bairro do Charqueir\u00e3o - Montijo (Terminal Fluvial), via Vale Porim"], ["4207", "Montijo (\u00c1rea Comercial) - Montijo (Terminal Fluvial)"]],
    "403": [["4203", "Afonsoeiro - Montijo (Terminal Fluvial), via Bairro da Liberdade"], ["4206", "Bairro Esteval - Montijo (Terminal Fluvial)"]],
    "404": [["4202", "Afonsoeiro - Bairro do Saldanha, via Bairro da Cal\u00e7ada"]],
    "410": [["4511", "Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio), via Samouco"], ["4600", "Alcochete (Freeport) - Barreiro (Terminal)"], ["4601", "Barreiro (Terminal) - Montijo (Terminal Rodovi\u00e1rio)"], ["4603", "Barreiro (Terminal) - Ch\u00e3o Duro"]],
    "412 (Adaptado)": [["4501", "Alcochete - Montijo (Terminal Fluvial)"]],
    "413": [["4510", "Alcochete (Freeport) - Montijo (Terminal Rodovi\u00e1rio)"], ["4512", "Alcochete (Freeport) - Set\u00fabal (ITS), via Alto Estanqueiro"], ["4513", "Alcochete (Freeport) - Pinhal Novo"], ["4517", "Montijo (Terminal Rodovi\u00e1rio) - Set\u00fabal (ITS)"], ["4523", "Montijo (Terminal Rodovi\u00e1rio) - Pinhal Novo"]],
    "414": [["4514", "Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es"]],
    "415": [["4504", "Montijo (Terminal Fluvial) - Passil"]],
    "416": [["4514", "Canha - Montijo (Terminal Rodovi\u00e1rio), via Peg\u00f5es"], ["4515", "Montijo (Terminal Rodovi\u00e1rio) - Peg\u00f5es"]],
    "419": [["4502", "Alcochete - Passil"]],
    "426": [["4516", "Montijo (Terminal Rodovi\u00e1rio) - Rio Frio"]],
    "431": [["4703", "Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio), via Alcochete e Samouco"]],
    "432": [["4702", "Lisboa (Oriente) - Valbom"], ["4704", "Atalaia - Lisboa (Oriente)"]],
    "435": [["4705", "Lisboa (Oriente) - Samouco"], ["4707", "Lisboa (Oriente) - Montijo (Terminal Rodovi\u00e1rio)"]],
    "440": [["4643", "Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)"]],
    "441": [["4210", "Canha - Foros Boavista"], ["4212", "Foros Boavista - Peg\u00f5es"]],
    "444": [["4520", "Faias - Peg\u00f5es"]],
    "446": [["4521", "Faias - Pinhal Novo"], ["4522", "Faias - Poceir\u00e3o"]],
    "447": [["4305", "Brejos do Assa - Palmela (Terminal)"], ["4524", "Palmela (Terminal) - Peg\u00f5es"]],
    "448": [["4322", "Pinhal Novo - Rio Frio"]],
    "449": [["4320", "Pinhal Novo | Circular"], ["4321", "Pinhal Novo - Qta do Anjo"]],
    "451": [["4530", "Bairro Vila Morena - Pinhal Novo"]],
    "453": [["4201", "Afonsoeiro - Bairro da Liberdade"], ["4706", "S\u00e3o Francisco - Lisboa (Oriente)"]],
    "454": [["4302", "Palmela (Esta\u00e7\u00e3o) - Palmela (Terminal)"], ["4307", "Loja Nova - Palmela (Terminal)"]],
    "455": [["4311", "Asseiceira - Poceir\u00e3o"]],
    "561": [["4725", "Lisboa (Sete Rios) - Set\u00fabal (ITS)"]],
    "562": [["4720", "Lisboa (Oriente) - Set\u00fabal (ITS)"]],
    "563": [["4715", "Lisboa (Oriente) - Set\u00fabal (ITS), via Pinhal Novo"]],
    "565": [["4710", "Lisboa (Oriente) - Palmela (Terminal)"], ["4711", "Lisboa (Oriente) - Pinhal Novo"]],
    "583": [["3610", "Brevemente | Cacilhas (Terminal) - Set\u00fabal (ITS), via A2"]],
    "601": [["4406", "Manteigadas - Set\u00fabal (Mercado)"], ["4408", "Manteigadas - Set\u00fabal (Mercado), via Bela Vista"], ["4412", "Morgada - Set\u00fabal (Mercado)"], ["4413", "Morgada - Set\u00fabal (Mercado), via Bela Vista"], ["4436", "Set\u00fabal (Mercado) - Set\u00fabal (Av. Soeiro Pereira Gomes)"]],
    "602": [["4416", "Po\u00e7o Mouro - Set\u00fabal (ITS)"], ["4417", "Po\u00e7o Mouro - Set\u00fabal (ITS), via Manteigadas"], ["4418", "Set\u00fabal (Alegro) - Set\u00fabal (Av. 5 Outubro)"], ["4420", "Set\u00fabal (Alegro) - Set\u00fabal (ITS)"]],
    "604": [["4403", "Fonte da Talha - Set\u00fabal (Av. Lu\u00edsa Todi)"], ["4423", "Amoreiras \u2013 Set\u00fabal (Av. Lu\u00edsa Todi)"], ["4551", "Palmela (USF) - Set\u00fabal (Av. Lu\u00edsa Todi)"]],
    "605": [["4431", "Set\u00fabal (ITS) - Set\u00fabal (Quinta Varzinha)"]],
    "607": [["4434", "Set\u00fabal (Mercado 2 de Abril) - Set\u00fabal (R. Timor)"]],
    "608": [["4424", "Set\u00fabal (Bairro Viso) - Manteigadas"], ["4425", "Set\u00fabal (Escola Viso) - Mitrena"], ["4426", "Set\u00fabal (Bairro Viso) - Set\u00fabal (CHEset\u00fabal)"]],
    "609": [["4421", "Set\u00fabal (Bairro Camolas) - Set\u00fabal (Casal Figueiras)"], ["4422", "Set\u00fabal (Bairro Camolas) - Set\u00fabal (Casal Figueiras), via Bairro do Viso"], ["4428", "Set\u00fabal (Casal Figueiras) - Vale Ana Gomes"]],
    "610": [["4438", "Set\u00fabal (Monte Belo Norte) - Set\u00fabal (Saboaria)"], ["4440", "Set\u00fabal (Monte Belo Norte) - Set\u00fabal (Saboaria), via Alegro"], ["4541", "Algeruz - Set\u00fabal (Av. Lu\u00edsa Todi)"]],
    "612": [["4419", "Brejos Canes - Set\u00fabal (Saboaria)"], ["4441", "Set\u00fabal (Saboaria) - Set\u00fabal (Vale Cobro)"]],
    "614": [["4421", "Set\u00fabal (Bairro Camolas) - Set\u00fabal (Casal Figueiras)"], ["4422", "Set\u00fabal (Bairro Camolas) - Set\u00fabal (Casal Figueiras), via Bairro do Viso"], ["4433", "Alto Guerra - Set\u00fabal (Casal Figueiras)"]],
    "616": [["4407", "Manteigadas - Set\u00fabal (Mercado), via Bairro da Carmona"]],
    "680": [["4301", "Palmela (Centro) - Palmela (Terminal)"]],
    "708": [["4545", "Biscainho - Set\u00fabal (Bela Vista)"]],
    "709": [["4901", "Landeira - Set\u00fabal (ITS)"]],
    "710": [["4310", "\u00c1guas de Moura - Poceir\u00e3o"]],
    "711": [["4312", "Poceir\u00e3o - Vale Abrunheira (X), via Fernando P\u00f3"]],
    "723": [["4472", "Praia do Creiro - Set\u00fabal (ITS)"], ["4476", "Praias Arr\u00e1bida | Circular"]],
    "725": [["4474", "Figueirinha - Set\u00fabal (Alegro)"]],
    "726": [["4471", "Praia Albarquel | Circular"]],
    "727": [["4470", "Brejos Azeit\u00e3o - Praia do Creiro"]],
    "751": [["4414", "Out\u00e3o (Hospital) - Set\u00fabal (ITS)"], ["4415", "Out\u00e3o (Hospital) - Set\u00fabal (ITS), via vale da Rasca"]],
    "754": [["4631", "Fogueteiro (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)"]],
    "755": [["4630", "Corroios (Esta\u00e7\u00e3o) - Set\u00fabal (ITS)"]],
    "756": [["4452", "Mitrena (Portucel) - Set\u00fabal (ITS)"]],
    "757": [["4453", "Mitrena (Portucel) - Set\u00fabal (ITS), via Estrada Gra\u00e7a"]],
    "758": [["4548", "Lagame\u00e7as - Set\u00fabal (ITS)"]],
    "764": [["4540", "\u00c1guas de Moura - Set\u00fabal (ITS)"], ["4544", "Bairro Marga\u00e7a - Set\u00fabal (ITS)"]],
    "765": [["4540", "\u00c1guas de Moura - Set\u00fabal (ITS)"], ["4544", "Bairro Marga\u00e7a - Set\u00fabal (ITS)"]],
    "766": [["4442", "Praias do Sado (Esta\u00e7\u00e3o) - Set\u00fabal (Bela Vista)"]],
    "767": [["4547", "Cabanas - Set\u00fabal (ITS)"], ["4549", "Palmela (Terminal) - Set\u00fabal (ITS)"], ["4562", "Set\u00fabal (ITS) - Vila Nogueira de Azeit\u00e3o, via Palmela (Esta\u00e7\u00e3o)"]],
    "768": [["4306", "Cabanas - Palmela (Terminal)"], ["4550", "Palmela (Terminal) - Vila Nogueira de Azeit\u00e3o"], ["4562", "Set\u00fabal (ITS) - Vila Nogueira de Azeit\u00e3o, via Palmela (Esta\u00e7\u00e3o)"]],
    "770": [["4641", "Quinta do Conde - Set\u00fabal (ITS)"]],
    "774": [["4546", "Biscainho - Set\u00fabal (ITS)"]],
    "776": [["4451", "Mitrena (Lisnave) - Set\u00fabal (ITS)"]],
    "779": [["4542", "Algeruz - Set\u00fabal (ITS)"], ["4543", "Algeruz - Set\u00fabal (ITS), via Po\u00e7oilos"]],
    "780": [["4402", "Estefanilha - Set\u00fabal (ITS)"], ["4437", "Faralh\u00e3o - Set\u00fabal (ITS)"]],
    "781": [["4411", "Morgada - Set\u00fabal (ITS)"]],
    "783": [["3605", "Cacilhas (Terminal) - Set\u00fabal (ITS), via Azeit\u00e3o"]],
    "797": [["4439", "Praias do Sado - Set\u00fabal (ITS)"]]
  }
};
const diretorio_pontos_navegante = {
  "Alameda": ["Alameda", "EstaÃ§Ã£o de Metro da Alameda", "", "Seg-Dom 6:30-01:00"],
  "Alcochete": ["Alcochete", "Biblioteca Municipal", "R. Prof. Leite da Cunha, 2890-087 Alcochete", "Ter 14:00-21:00, Qua-Sab 10:30-18:30"],
  "Almada": ["Almada", "Terminal Fluvial de Cacilhas", "Largo Alfredo Dinis, 2800-252 Almada", ""],
  "Amadora": ["Amadora", "EdifÃ­cio PaÃ§os do Concelho", "PraÃ§a do MunicÃ­pio, 1100-038 Lisboa", "Seg-Sex 09:00-18:00"],
  "Barreiro": ["Barreiro", "Mercado Municipal 1Âº de Maio", "R. EÃ§a de Queiroz", "Seg-Sex 07:00-14:00/16:30-19:30, Sab 07:00-16:00"],
  "Cascais": ["Cascais", "Loja de CidadÃ£o", "R. Manuel Joaquim de Avelar 118, 2750-421 Cascais", "Seg-Sex 09:00-18:01"],
  "Lisboa": ["Lisboa", "CÃ¢mara Municipal de Lisboa", "Campo Grande 25, 1749-099 Lisboa", "Seg-Sex 08:00-20:00"],
  "Loures": ["Loures", "Loures Shopping", "Av. Descobertas 90, 2670-457 Loures", "Seg-Dom 08:00-23:00"],
  "Mafra": ["Mafra", "Loja do CidadÃ£o", "Av. 25 de Abril 5, 2640-456 Mafra", "Seg-Sex 09:00-17:00"],
  "Moita": ["Moita", "BalcÃ£o do MunÃ­cipe Baixa da Banheira", "Av. Humberto Delgado 7A, 2860-027 Alhos Vedros", "Seg-Sex 09:00-12:30/14:00-17:30"],
  "Montijo": ["Montijo", "Mercado Municipal do Montijo", "PraÃ§a Gomes Freire de Andrade 29, 2870-237 Montijo", "Seg-Dom 09:00-22:00"],
  "Odivelas": ["Odivelas", "Loja de CidadÃ£o Centro Comercial Strada", "Estr. da PaiÃ£, 2675-626 Odivelas", "Seg-Sex 08:30-19:30, Sab 09:30-15:00"],
  "Oeiras": ["Oeiras", "Centro Comercial Oeiras Parque", "Av. AntÃ³nio Bernardo Cabral de Macedo, 2770-219 Oeiras", "Seg-Dom 10:00-23:00"],
  "Palmela": ["Palmela", "Mercado Municipal de Pinhal Novo", "PraÃ§a da IndepÃªndencia, 2955-120 Pinhal Novo", "Seg-Sex 07:30-18:00, Sab-Dom 07:30-13:30"],
  "Pragal": ["Pragal", "EstaÃ§Ã£o da Pragal Fertagus", "", "Seg-Sex 05:20-01:45, Sab 05:20-01:00, Dom 05:40-01:00"],
  "Seixal": ["Seixal", "Loja do MunÃ­cipe no RioSul Shopping", "Av. Libertadores de Timor Loro Sae, 2840-168 Seixal", "Seg-Sab 10:00-20:00"],
  "Sesimbra": ["Sesimbra", "BalcÃ£o Ãºnico de serviÃ§o no EdifÃ­cio da PresidÃªncia", "EdifÃ­cio do Mercado Municipal, R. Manuel de Arriaga, 2975-329 Q.ta do Conde", "Seg-sex 08:00-19:00, Sab 08:30-13:00"],
  "SetÃºbal": ["SetÃºbal", "Centro Comercial Alegro SetÃºbal", "Av. Antero de Quental 2, 2910-394 SetÃºbal", "Seg-Dom 08:30-23:00"],
  "Sintra": ["Sintra", "Junta de Freguesia de Rio de Mouro", "Rua de Oscar Monteiro Torres 19 R/C A, 2635-385 Rio de Mouro", "Seg/Qua-Sex 09:00-13:00/14:30-17:30, Ter 09:00-13:00/14:30-20:00"],
  "Vila Franca de Xira": ["Vila Franca de Xira", "Biblioteca Municipal de Alverca do Ribatejo", "Centro Comercial Parque Piso 1, Alverca do Ribatejo", "Ter 10:00-22:00, Qua-Sex 10:00-19:00, Sab 10:00-13:00/14:00-17:30"]
};
// visualizador de horÃ¡rios
// Universidade LusÃ³fona, DEISI, 2022
var today;
var ficheiro;
var proxima_passagem;

function da_cor(linha) {
  var cor;
  var tarifario = linhas_e_tarifario[linha];

  switch (tarifario) {
    case 'RÃ¡pida':
      cor = '#ffb005';
      break;

    case 'Longa':
      cor = '#ff0047';
      break;

    case 'Mar':
      cor = '#3dff9e';
      break;

    case 'PrÃ³xima':
      cor = '#4099ff';
      break;

    case 'Inter-regional':
      cor = '#bd1aff';
      break;

    case 'TurÃ­stica':
      cor = '#ff5900';
      break;
  }

  return cor;
}
/* mostra hora e dia **************************************************************/


function mostraHora() {
  // data de hoje
  var date_now = new Date();
  const options = {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  };
  var e = date_now.toLocaleDateString('pt-PT', options);
  var year = date_now.getFullYear();
  var month = ('0' + (date_now.getMonth() + 1)).slice(-2);
  var day = ('0' + date_now.getDate()).slice(-2);
  var hours = '' + date_now.getHours();
  var minutes = ('00' + date_now.getMinutes()).slice(-2);
  today = parseInt(`${year}${month}${day}`, 10); // today = 20220704;
  // month = '07';
  // day = '04';
  // year = '2022';

  hora_agora = parseInt(`${hours}${minutes}`, 10);
  const meses = ["Janeiro", "Fevereiro", "MarÃ§o", "Abril", "Maio", "Junho", "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];
  document.querySelector('#date').innerHTML = `<b>${hours}h${minutes}</b>,  ${day} de ${meses[month - 1]} de ${year}`;
  return [today, hora_agora];
}
/* mostra hora e dia **************************************************************/


function mostraHoraEDiaNosInputs() {
  var date_now = new Date().toLocaleDateString('pt-PT').replaceAll('/', '-');
  var time_now = new Date().toLocaleTimeString('pt-PT').substring(0, 5);
  var year = date_now.substring(6);
  var month = date_now.substring(3, 5);
  var day = date_now.substring(0, 2);
  date_now = year + '-' + month + '-' + day;
  document.querySelector('#timeInput').value = time_now;
  document.querySelector('#dateInput').value = date_now;
  today = parseInt(date_now.replaceAll('-', ''), 10);
  var hora_agora = parseInt(time_now.replace(':', ''), 10); // today = 20220704;
  // month = '07';
  // day = '04';
  // year = '2022';

  return [today, hora_agora];
}
/* obter hora e data inserida pelo utilizador ****************************************/


function getHoraDiaInserida() {
  var dataInserida = document.querySelector('#dateInput').value;
  var horaInserida = document.querySelector('#timeInput').value;
  hora_agora = parseInt(horaInserida.replace(':', ''), 10);
  today = parseInt(dataInserida.replaceAll('-', ''), 10);
  document.querySelector('#paragens').innerHTML = '';
  document.querySelector('#variante').innerHTML = '';
  mostraLinha();
}
/*************************************************************************************/

/* cria elemento select com as linhas existentes *************************************/

/*************************************************************************************/


function mostraSelectLinhas() {
  // esconde estrela
  document.querySelector('#linhaPreferida').style.display = 'none';
  var datalist = document.querySelector('#datalistLinhas');

  for (const numero in linhas_e_seus_horarios_nomes_ficheiros) {
    const option = document.createElement("option");
    datalist.appendChild(option);
    option.setAttribute('value', numero);
    let texto = document.createTextNode(`${numero}`);
    option.appendChild(texto);
  }
}
/*************************************************************************************/

/* cria elemento com rotas ***********************************************************/

/*************************************************************************************/


function mostraLinha() {
  document.querySelector('#inputLinha').blur();
  var element = document.getElementById('horariosSeparador');
  var headerOffset = 40;
  var elementPosition = element.getBoundingClientRect().top;
  var offsetPosition = elementPosition + window.pageYOffset - headerOffset;
  window.scrollTo({
    top: offsetPosition,
    behavior: "smooth"
  }); // document.querySelector('#horariosSeparador').scrollIntoView({
  //     block: 'start',
  //     behavior: 'smooth',
  //   });

  var linha = document.querySelector('#inputLinha').value; // se a linha nÃ£o existir, apaga tudo e vaiSE embora

  if (!(linha in linhas_e_seus_horarios_nomes_ficheiros) || Object.keys(linhas_e_seus_horarios_nomes_ficheiros[linha]).length == 0) {
    document.querySelector('#paragens').innerHTML = '';
    document.querySelector('#variante').innerHTML = `NÃ£o existe horÃ¡rio para a linha ${linha}.`;
    document.querySelector('#infolinha').style.display = 'none';
    document.querySelector('#comentarioInfo').style.display = 'none';
    document.querySelector('#inputLinha').style.backgroundColor = '#ffdd00';
    document.querySelector('#linhaPreferida').style.display = 'none';
    return 0;
  }

  document.querySelector('#infolinha').style.display = 'flex';
  var cor;
  var tarifario = linhas_e_tarifario[linha];

  switch (tarifario) {
    case 'RÃ¡pida':
      cor = '#ffb005';
      break;

    case 'Longa':
      cor = '#ff0047';
      break;

    case 'Mar':
      cor = '#3dff9e';
      break;

    case 'PrÃ³xima':
      cor = '#4099ff';
      break;

    case 'Inter-regional':
      cor = '#bd1aff';
      break;

    case 'TurÃ­stica':
      cor = '#ff5900';
      break;
  } // mostra estrela


  const button = document.querySelector('#linhaPreferida');
  button.style.display = 'block';
  button.value = linha;

  if ('listaLinhasPreferidas' in localStorage && localStorage.getItem('listaLinhasPreferidas').includes(linha)) {
    button.querySelector('span').style.color = '#ffdd00';
  } else {
    button.querySelector('span').style.color = 'lightgrey';
  }

  button.onclick = () => {
    adiciona_favoritos(button);
    linhasPreferidas();
  }; // insere o select dentro do elemento <span id="variante">


  document.querySelector('#variante').innerHTML = '';
  const varianteDiv = document.querySelector('#variante');
  var primeiro = true;

  for (const nome in linhas_e_seus_horarios_nomes_ficheiros[linha]) {
    const label = document.createElement("label");
    const nome_ficheiro = linhas_e_seus_horarios_nomes_ficheiros[linha][nome];
    const input = document.createElement("input");
    input.setAttribute('type', 'radio');
    input.setAttribute('name', 'rota');
    input.setAttribute('value', nome_ficheiro);
    label.appendChild(input);
    const div = document.createElement('div');
    div.innerHTML = nome; // div.style.marginLeft = '5px';
    // div.style.fontWeight = 'normal';
    // div.style.fontSize = '1.4em';

    label.appendChild(div);
    const rota = nome_ficheiro.slice(0, -5);

    if (rota in horarios_pdf) {
      const div_pdf = document.createElement('div');
      label.appendChild(div_pdf);
      const a = document.createElement('a');
      a.setAttribute('href', `images/horarios_pdf/${horarios_pdf[rota]}`);
      a.setAttribute('target', 'blank');
      div_pdf.appendChild(a);
      const img = document.createElement('img');
      img.setAttribute('src', 'images/save_to_pdf.png');
      a.appendChild(img);
    }

    varianteDiv.appendChild(label);

    if (primeiro) {
      input.setAttribute('checked', true);
      div.style.fontWeight = 'bold';
      criaHorario(nome_ficheiro);
      primeiro = false;
    }
  }
  /* eventlistener para o caso de escolher outra variante  */


  document.querySelectorAll('input[name="rota"]').forEach(input => {
    input.onclick = () => {
      document.querySelectorAll('#variante div').forEach(p => {
        p.style.fontWeight = 'normal';
      });
      input.nextSibling.style.fontWeight = 'bold';
      var ficheiro = input.value;
      criaHorario(ficheiro);
    };
  });
}
/*************************************************************************************/

/* cria horario escolhido ************************************************************/

/*************************************************************************************/


function criaHorario(ficheiro) {
  document.querySelector('#comentarioInfo').style.display = 'block';
  document.querySelector('#meuHorarioEscolhas').innerHTML = '';
  document.querySelector('#paragens').style.display = "block";
  criaHorarioProximo(ficheiro);
  document.querySelector('#legenda_tipos_linha').style.display = 'block';
}
/*************************************************************************************/

/* cria lista paragens com prÃ³ximas passagens ****************************************/

/*************************************************************************************/


function abrePrimeiraParagem(cor) {
  paragem = document.querySelector('table#espinha tr:first-child .paragem');
  paragem.querySelector('.nome').style.fontWeight = 'bold';
  paragem.style.backgroundColor = cor;
  paragem.style.borderRadius = '1em'; //   paragem.style.border = '1px solid black';

  document.querySelector('table#espinha tr:first-child').querySelector('.bola').style.visibility = 'visible';
  let h = paragem.querySelector('.horario');
  h.style.display = "block"; //  paragem.style.border = '1px solid black';
}
/*************************************************************************************/

/* criaHorarioProximo                                                 ****************/

/*************************************************************************************/


function criaHorarioProximo(ficheiro) {
  var [today, hora_agora] = mostraHoraEDiaNosInputs();
  /* insere a hora e o dia*/

  var anterior, acumulado, diferenca, contador;
  var cor;
  document.getElementById("horario").innerHTML = ''; // if (today < 20220701 && !((ficheiro.slice(0,3) >= '440' && ficheiro.slice(0,3) < '447') || ficheiro.slice(0,4) == '4551')) 
  // {
  //     document.querySelector('#comentarioInfo').innerHTML = 'Clique na paragem e saiba o prÃ³ximo autocarro (vÃ¡lido a partir de 1 julho)'; // Clique na paragem para saber o prÃ³ximo autocarro
  // } 
  // else if (today < 20220704 && ((ficheiro.slice(0,3) >= '440' && ficheiro.slice(0,3) < '447') || ficheiro.slice(0,4) == '4551')) 
  // {
  //     document.querySelector('#comentarioInfo').innerHTML = 'Clique na paragem e saiba o prÃ³ximo autocarro  (vÃ¡lido a partir de 4 julho)';
  // }
  // else 
  // {
  //     document.querySelector('#comentarioInfo').innerHTML = 'Clique na paragem e saiba o prÃ³ximo autocarro';
  // }

  meuHorarioEscolhas.innerHTML = '';
  /* vai buscar o horario */

  fetch('images/horarios/' + ficheiro).then(response => response.json())
  /** novo com tabela  ********************************************** */
  .then(horario => {
    var tarifario = linhas_e_tarifario[ficheiro.slice(0, 4)];

    switch (tarifario) {
      case 'RÃ¡pida':
        cor = '#ffb005';
        break;

      case 'Longa':
        cor = '#ff0047';
        break;

      case 'Mar':
        cor = '#3dff9e';
        break;

      case 'PrÃ³xima':
        cor = '#4099ff';
        break;

      case 'Inter-regional':
        cor = '#bd1aff';
        break;

      case 'TurÃ­stica':
        cor = '#ff5900';
        break;
    }

    document.querySelector('#inputLinha').style.backgroundColor = cor;
    document.querySelector('#inputLinha').style.border = '';
    /*****************************************************************/

    /** B. insere horario ********************************************/

    /*****************************************************************/

    const paragem = 0;
    document.getElementById("horario").innerHTML = '';
    const horario_html = cria_horario(ficheiro, horario, paragem);
    document.getElementById("horario").appendChild(horario_html);
    mostra_legenda_periodos();
    /*****************************************************************/

    /** B. cria espinha ********************************************/

    /*****************************************************************/
    // cria divs com as paragens e horario de cada

    var counter = 0;
    const paragens = document.getElementById("paragens");
    paragens.innerHTML = '';
    const descricaoEspinha = document.createElement('div');
    descricaoEspinha.innerHTML = `<span class="titulo-descricao">Clique numa paragem</span><br> e saiba o prÃ³ximo autocarro`;
    descricaoEspinha.setAttribute('id', 'descricaoEspinha'); // descricaoEspinha.style.color = cor;

    paragens.appendChild(descricaoEspinha);
    const table = document.createElement('table');
    table.setAttribute('id', 'espinha');
    paragens.appendChild(table);
    var primeira = true;
    horario.forEach(paragem => {
      const tr = document.createElement('tr');
      table.appendChild(tr); //*  cria elemento td tempo: calcula tempo entre paragens *********************************

      td = document.createElement('td');
      td.setAttribute('class', 'tempo');
      td.style.fontSize = '0.9em';

      if (primeira) {
        contador = 0;
        anterior = parseInt(paragem[1][0][0].replace(':', ''), 10);
        acumulado = 0;
        let img = document.createElement('img');
        img.setAttribute('src', 'images/clock.png');
        td.appendChild(img);
      }

      diferenca = parseInt(paragem[1][0][0].replace(':', ''), 10) - anterior;
      acumulado += diferenca; // td.innerHTML = parseInt(paragem[1][0][0].replace(':',''), 10) + ', ' + anterior + ', ' + diferenca + ', ' + acumulado;

      if (contador++ != 0 && contador % 4 == 0) {
        td.innerHTML = '<hr>';
        document.querySelector(`#espinha tr:nth-child(${contador - 2}) td:first-child`).innerHTML = acumulado;
        acumulado = 0;
      }

      anterior = parseInt(paragem[1][0][0].replace(':', ''), 10);
      tr.appendChild(td); //*  cria elemento td cor *********************************

      var td = document.createElement('td');
      td.setAttribute('class', 'cor');
      td.style.backgroundColor = cor;

      if (primeira) {
        td.style.borderRadius = '1em 1em 0 0';
        var imgs = document.createElement('img');
        imgs.setAttribute('src', 'images/bullet-start.png');
        imgs.style.width = '1.4em';
        imgs.style.verticalAlign = 'center';
        td.appendChild(imgs);
        const bolaSpan = document.createElement('span');
        bolaSpan.setAttribute('class', 'bola');
        td.appendChild(bolaSpan);
      } else {
        const bolaSpan = document.createElement('span');
        bolaSpan.setAttribute('class', 'bola');
        var imgc = document.createElement('img');
        imgc.setAttribute('src', 'images/bullet-current.png');
        imgc.style.width = '1.4em';
        imgc.style.verticalAlign = 'center';
        bolaSpan.appendChild(imgc);
        td.appendChild(bolaSpan);
      }

      tr.appendChild(td);
      td = document.createElement('td');
      td.setAttribute('class', 'hr');
      const hr = document.createElement('hr');
      td.appendChild(hr);
      tr.appendChild(td);
      td = document.createElement('td');
      td.setAttribute('class', 'paragem paragemProximo');
      td.setAttribute('data-paragem', `${counter++}`);
      tr.appendChild(td); //** elemento que guarda info da paragem: hr e nome *************** */

      const paragemDiv = document.createElement('div');
      paragemDiv.setAttribute('class', 'paragemInfo');
      td.append(paragemDiv);
      const nomeSpan = document.createElement("div");
      nomeSpan.setAttribute('class', 'nome');
      const nome = document.createTextNode(paragem[0]);
      nomeSpan.appendChild(nome);
      paragemDiv.appendChild(nomeSpan);

      if (primeira) {
        primeira = false;
      } //** elemento que guarda info das proximas passagens *************** */
      // verifica horario das proximas carreiras


      var listaProximas = [];

      for (let i = 0; i < paragem[1].length; i++) {
        var hora_horario = parseInt(paragem[1][i][0].replace(":", "")); // if (hora_horario > hora_agora)

        if (date_service_ids[today].includes('' + paragem[1][i][1]) && hora_horario > hora_agora) {
          listaProximas.push(paragem[1][i][0]);
        }
      }

      listaProximas.sort(function (a, b) {
        return parseInt(a.replace(':', ''), 10) - parseInt(b.replace(':', ''), 10);
      }); // cria elemento com o horario

      const horarioDiv = document.createElement("div");
      horarioDiv.setAttribute('class', 'horario');
      horarioDiv.style.display = "none"; // span com hora da proxima passagem na paragem escolhida

      const proximaSpan = document.createElement("span");
      proximaSpan.setAttribute('class', 'proxima');
      proximaSpan.style.fontSize = '0.85em';
      proximaSpan.style.fontWeight = 'bold'; // proximaSpan.style.fontWeight = "bold";

      var proxima;

      if (listaProximas.length > 0) {
        proxima_passagem = listaProximas[0];
        proxima = document.createTextNode('PrÃ³ximos: ' + listaProximas[0]);
      } else {
        if (today < 20220701 && ficheiro[0] == '3') {
          proxima = document.createTextNode('HorÃ¡rio vÃ¡lido sÃ³ a partir de 1 julho.');
        } else {
          proxima = document.createTextNode('hoje esta linha jÃ¡ nÃ£o circula mais');
        }
      }

      proximaSpan.appendChild(proxima);
      horarioDiv.appendChild(proximaSpan); // div com horas das 2 seguintes passagens na paragem escolhida

      const seguintesSpan = document.createElement("span");
      seguintesSpan.setAttribute('class', 'seguintes');
      seguintesSpan.style.fontSize = '0.85em';
      seguintesSpan.style.fontWeight = 'bold';
      seguintesSpan.style.padding = "0 0 0 1em";
      var seguintes = document.createTextNode('');

      if (listaProximas.length == 1) {
        seguintes = document.createTextNode(`(Ãºltimo)`);
      } else if (listaProximas.length == 2) {
        seguintes = document.createTextNode(`${listaProximas[1]}`);
      } else if (listaProximas.length > 2) {
        seguintes = document.createTextNode(`${listaProximas[1]}, ${listaProximas[2]}`);
      }

      seguintesSpan.appendChild(seguintes);
      horarioDiv.appendChild(seguintesSpan);
      td.appendChild(horarioDiv);
    });
    var imge = document.createElement('img');
    imge.setAttribute('src', 'images/bullet-end.png');
    imge.style.verticalAlign = 'center';
    imge.style.width = '1.4em';
    table.querySelector('tr:last-child td:nth-child(2) .bola').innerHTML = '';
    table.querySelector('tr:last-child td:nth-child(2)').appendChild(imge);
    table.querySelector('tr:last-child td:nth-child(2)').style.borderRadius = '0 0 1em 1em';

    if (contador % 4 > 2) {
      document.querySelector(`#espinha tr:nth-child(${contador - 2}) td:first-child`).innerHTML = acumulado;
    } else if (contador % 4 > 1) {
      document.querySelector(`#espinha tr:nth-child(${contador - 1}) td:first-child`).innerHTML = acumulado;
    }

    document.querySelectorAll('.paragem').forEach(p => {
      p.parentElement.querySelector('.bola').style.visibility = 'hidden';
    });
    abrePrimeiraParagem(cor); // document.querySelectorAll('.paragem').forEach(paragem => {
    //     paragem.onmouseover = () => {
    //         if (paragem.parentElement.querySelector('.nome').style.fontWeight != 'bold') {
    //             paragem.style.backgroundColor = '#fff5b3';
    //             paragem.style.borderRadius = '1em';
    //             paragem.parentElement.querySelector('.bola').style.visibility = 'visible';
    //             paragem.parentElement.querySelector('.bola').style.opacity = '0.2';
    //         }
    //     }
    //     paragem.onmouseout = () => {
    //         if (paragem.parentElement.querySelector('.nome').style.fontWeight != 'bold') {
    //             paragem.style.backgroundColor = '';
    //             paragem.parentElement.querySelector('.bola').style.visibility = 'invisible';
    //             paragem.parentElement.querySelector('.bola').style.opacity = '0.0';
    //         }
    //     }
    // });

    document.querySelectorAll('#espinha > *').forEach(tr => {
      tr.onmouseover = () => {
        if (tr.querySelector('.nome').style.fontWeight != 'bold') {
          tr.childNodes[3].style.backgroundColor = cor;
          tr.childNodes[3].style.opacity = '0.8';
          tr.childNodes[3].style.borderRadius = '1em';
          tr.childNodes[3].style.fontSize = '1em';
          tr.querySelector('.bola').style.visibility = 'visible';
          tr.querySelector('.bola').style.opacity = '0.5';
        }
      };

      tr.onmouseout = () => {
        if (tr.querySelector('.nome').style.fontWeight != 'bold') {
          tr.childNodes[3].style.backgroundColor = '';
          tr.childNodes[3].style.opacity = '1';
          tr.childNodes[3].style.fontSize = '0.9em';
          tr.querySelector('.bola').style.visibility = 'invisible';
          tr.querySelector('.bola').style.opacity = '0.0';
        }
      };
    });
    document.querySelectorAll('.paragemProximo').forEach(paragem => {
      paragem.onclick = () => {
        document.querySelectorAll('.paragemProximo').forEach(p => {
          p.parentElement.querySelector('.bola').style.visibility = 'hidden';
          p.style.backgroundColor = '';
          p.querySelector('.horario').style.display = "none";
          p.querySelector('.nome').style.fontWeight = 'normal';
          p.querySelector('.nome').style.fontSize = '0.9em';
        });
        paragem.parentElement.querySelector('.bola').style.visibility = 'visible';
        paragem.parentElement.querySelector('.bola').style.opacity = '1.0';
        paragem.style.backgroundColor = cor;
        paragem.style.opacity = '1.0';
        paragem.querySelector('.nome').style.fontSize = '1em';
        paragem.querySelector('.nome').style.fontWeight = 'bold';
        paragem.querySelector('.nome').style.whiteSpace = 'wrap';
        paragem.style.borderRadius = '1em';
        paragem.querySelector('.horario').style.display = "block";
        document.getElementById("horario").innerHTML = '';
        const horario_html = cria_horario(ficheiro, horario, parseInt(paragem.dataset.paragem, 10));
        document.getElementById("horario").appendChild(horario_html);
        mostra_legenda_periodos();
      };
    });
  });
}

function mostra_legenda_periodos() {
  const info = document.querySelector('.periodos_datas');
  info.style.display = 'none';

  document.querySelector('.descricao_periodos').onmouseover = () => {
    info.style.display = 'block';
  };

  document.querySelector('.descricao_periodos').onmouseout = () => {
    info.style.display = 'none';
  };
}
/*************************************************************************************/

/* cria horario completo para linha e paragem  ***************************************/

/*************************************************************************************/


function cria_horario(ficheiro, horario, paragem) {
  const horario_paragem = horario[paragem][1];
  const horario_paragem_por_dia = [[], [], [], [], [], [], [], [], []];
  /*** extrai hora e service_id duma paragem *******************************/

  for (let i = 0; i < horario_paragem.length; i++) {
    const hora = horario_paragem[i][0];
    const service_id = horario_paragem[i][1]; // tipo de dia (DU_VER, SAB_VER etc...)

    for (let tipo_dia = 0; tipo_dia < 9; tipo_dia++) {
      // guarda a hora no horario onde service_id Ã© vÃ¡lido
      if (plano_oferta[service_id][tipo_dia] != '0') {
        horario_paragem_por_dia[tipo_dia].push(hora);
      }
    }
  }

  const horario_refinado = [];

  for (let t = 0; t < horario_paragem_por_dia.length; t++) {
    const tipo_dia = horario_paragem_por_dia[t];
    const dia = [];

    for (let h = 0; h < 26; h++) {
      /////// h=0; h<25
      dia.push([]);
    }

    for (let i = 0; i < tipo_dia.length; i++) {
      const hora_minuto = tipo_dia[i];
      const hora = parseInt(hora_minuto.slice(0, 2));
      const minuto = hora_minuto.split(':')[1];
      dia[hora - 3].push(minuto); /////// hora - 1
    }

    horario_refinado.push(dia);
  } // ordena os minutos de cada hora de cada tipo_dia


  for (let tipo_dia = 0; tipo_dia < horario_refinado.length; tipo_dia++) {
    for (let h = 0; h < 26; h++) {
      horario_refinado[tipo_dia][h].sort();
    }
  }

  const horario_invertido = [];

  for (let tipo_dia = 0; tipo_dia < horario_refinado.length; tipo_dia++) {
    var maximo = 0;

    for (let h = 0; h < 26; h++) {
      if (horario_refinado[tipo_dia][h].length > maximo) {
        maximo = horario_refinado[tipo_dia][h].length;
      }
    }

    const dia = [];

    for (let h = 0; h < 26; h++) {
      /////// h=0; h<25
      const mins = [];

      for (let m = 0; m < maximo; m++) {
        mins.push(' ');
      }

      dia.push(mins);
    }

    for (let h = 0; h < horario_refinado[tipo_dia].length; h++) {
      for (let m = 0; m < horario_refinado[tipo_dia][h].length; m++) {
        dia[h][m] = horario_refinado[tipo_dia][h][m];
      }
    }

    horario_invertido.push(dia);
  }
  /*********************************************************************************/

  /***** cria elemento tabela ******************************************************/

  /*********************************************************************************/


  var periodo_str, dia_str;
  const periodos_str_list = ['PerÃ­odo Escolar', 'PerÃ­odo FÃ©rias Escolares', 'PerÃ­odo de VerÃ£o'];
  const dias_str_list = ['Dias Ãšteis', 'SÃ¡bados', 'Domingos'];
  const divGlobal = document.createElement('div');
  divGlobal.setAttribute('id', 'divGlobal');
  /***** Insere descriÃ§Ã£o do horario *********************************************/

  const descricaoHorario = document.createElement('div');
  descricaoHorario.innerHTML = `<span class='titulo-descricao'><b>${horario[paragem][0]}</b></span><br>HorÃ¡rio previsto de passagem`; // Paragem: 

  descricaoHorario.setAttribute('id', 'descricaoHorario');
  const linha = ficheiro.slice(0, 4); // descricaoHorario.style.color = da_cor(linha);

  divGlobal.appendChild(descricaoHorario);
  var legenda = {};

  for (let tipo_dia = 0; tipo_dia < horario_invertido.length; tipo_dia++) {
    if (horario_invertido[tipo_dia][0].length == 0) {
      continue;
    }

    const tipoDia = document.createElement('div');
    divGlobal.appendChild(tipoDia);
    /*****************************************************************************/

    /***   insere nome do perÃ­odo e do tipo de dia *******************************/

    /*****************************************************************************/

    if (tipo_dia % 3 == 0) {
      const titulo = document.createElement('div');
      titulo.setAttribute('class', 'periodo');
      periodo_str = periodos_str_list[Math.floor(tipo_dia / 3)];
      texto = document.createTextNode(periodo_str);
      titulo.appendChild(texto); // titulo.innerHTML = periodo_str;

      tipoDia.appendChild(titulo);
    }

    const subtitulo = document.createElement('div');
    subtitulo.setAttribute('class', 'tipodia');
    dia_str = dias_str_list[tipo_dia - Math.floor(tipo_dia / 3) * 3];
    hr = document.createElement('hr');
    subtitulo.appendChild(hr);
    texto = document.createTextNode(dia_str);
    subtitulo.appendChild(texto);
    tipoDia.appendChild(subtitulo);
    /*****************************************************************************/
    //*** cria elemento tabela **************************************************//

    /*****************************************************************************/

    const table = document.createElement('table');
    table.setAttribute('class', 'tabelaHorario');
    table.style.marginLeft = '0'; // table.style.border = "1px solid black"; //*********************************** tirar no fim

    tipoDia.appendChild(table);
    /*****************************************************************************/
    //*** linha das horas da tabela *********************************************//

    /*****************************************************************************/

    const trHour = document.createElement('tr');
    var th = document.createElement('th');
    th.innerHTML = 'Hora';
    th.style.width = '4.5ch';
    th.style.padding = '0 1ch';
    th.style.borderRadius = '1ch 0 0 1ch';
    th.style.textAlign = 'left';
    trHour.appendChild(th);

    for (let h = 4; h <= 24; h++) {
      // h=0; h<25
      th = document.createElement('th');
      th.innerHTML = h;
      trHour.appendChild(th);
    }

    th = document.createElement('th');
    th.innerHTML = 1;
    trHour.appendChild(th);
    th = document.createElement('th');
    th.innerHTML = 2;
    trHour.appendChild(th);
    th = document.createElement('th');
    th.innerHTML = 3;
    trHour.appendChild(th);
    th.style.paddingRight = '1ch';
    th.style.fontWidth = '3.5ch';
    th.style.borderRadius = '0 1ch 1ch 0';
    table.appendChild(trHour);
    /*****************************************************************************/
    //*** linhas dos minutos da tabela ******************************************//

    /*****************************************************************************/

    var primeira = true;

    for (let m = 0; m < horario_invertido[tipo_dia][0].length; m++) {
      const trMin = document.createElement('tr');

      for (let h = 0; h < 26; h++) {
        const td = document.createElement('td');

        if (primeira) {
          texto = document.createTextNode('Min.');
          primeira = false;
          td.appendChild(texto);
          td.style.textAlign = 'left';
          td.style.paddingLeft = '1ch';
          trMin.appendChild(td);
        } else {
          const minuto = horario_invertido[tipo_dia][h][m];
          /************************************************** */

          /** letra                                        */

          /************************************************** */

          const letra = minuto[minuto.length - 1];

          if (/^[A-Z]$/i.test(letra)) {
            texto = document.createTextNode(horario_invertido[tipo_dia][h][m].slice(0, 2));
            td.appendChild(texto);
            sup = document.createElement('sup');
            texto = document.createTextNode(letra);
            sup.appendChild(texto);
            td.appendChild(sup);
            legenda[letra] = plano_oferta_legenda[letra];
          } else {
            texto = document.createTextNode(horario_invertido[tipo_dia][h][m]);
            td.appendChild(texto); // if (
            //     data_periodo_dia['' + today][0] == tipo_dia && 
            //     (h+3) == parseInt(proxima_passagem.split(':')[0]) &&
            //     proxima_passagem.slice(-2) == minuto
            //     ) 
            //     {
            //         td.style.background = 'red';
            //     }
          }

          trMin.appendChild(td);
        }
      }

      table.appendChild(trMin);
    }
  }

  if (Object.keys(legenda).length != 0) {
    var div = document.createElement('div');

    for (letra in legenda) {
      const b = document.createElement('b');
      const texto = document.createTextNode(letra + ') ');
      b.appendChild(texto);
      div.appendChild(b);
      const span = document.createElement('span');
      span.innerHTML = legenda[letra] + '; ';
      div.appendChild(span);
    }

    divGlobal.appendChild(div);
  } // <img src="images/info.png" style="height:1em"> 


  var periodos = document.createElement('div');
  periodos.setAttribute('class', 'descricao_periodos');
  periodos.innerHTML = '<div class="periodos_info"><div><img src="images/calendar-search.png"></div><div> Consulte as datas respeitantes aos perÃ­odos indicados (escolar, de fÃ©rias escolares e de verÃ£o) </div></div>' + '<div class="periodos_datas">' + '   - <b>Periodo Escolar</b>: 2022: 3.1-7.4, 22.4-23.6, 16.9-16.12 | 2023: 3.1-17.2, 23.2-24.3, 10.4-18.6<br>' + '   - <b>Periodo de FÃ©rias Escolares</b>: 2022: 1-3.1, 8-21.4, 24-30.6, 1-15.9, 17-31.12 | 2023: 1-3.1, 18-23.2, 25.3-9.4, 19-30.6<br>' + '   - <b>Periodo de VerÃ£o</b>: 1.7-31.8<br>';
  '</div>';
  divGlobal.appendChild(periodos);
  return divGlobal;
}
/*************************************************************************************/

/* mostra botoes e ativa horario escolhido  ******************************************/

/*************************************************************************************/


function mostraBotoes() {
  // esconde tudo enquanto nÃ£o seja selecionada a linha
  document.querySelector('#comandosControlo').style.display = 'none';
  document.querySelector('#comentarioComandos').style.display = 'none';
  document.querySelector('#comentarioInfo').style.display = 'none';
  document.querySelector('#paragens').style.display = 'none';
  document.querySelectorAll('#comandosControlo button').forEach(button => {
    button.onclick = () => {
      // poe os outros a cinza
      document.querySelectorAll('#comandosControlo button').forEach(b => {
        b.style.backgroundColor = 'whitesmoke';
        b.style.fontWeight = 'normal';
        b.style.border = "1px solid black";
      }); // poe o clicado a branco

      button.style.backgroundColor = 'white';
      button.style.color = 'black';
      button.style.fontWeight = 'bold';
      button.style.border = "1px solid black"; // identifica qual o que estÃ¡ ativo. esconde/mostra botÃ£o save

      switch (button.dataset.tipo) {
        case 'proximo':
          document.querySelector('#tipoHorario').innerHTML = 'proximo';
          break;

        case 'meuHorario':
          document.querySelector('#tipoHorario').innerHTML = 'meuHorario';
      }

      mostraLinha();
    };
  });
}
/*************************************************************************************/

/* grava horario em PDF  *************************************************************/

/*************************************************************************************/


function printFunction() {
  const horario = document.querySelector('#toprint');
  horario.innerHTML = '';
  logo = document.createElement('img');
  logo.setAttribute('src', 'images/logo.png');
  const header = document.createElement('div');
  header.setAttribute('class', 'headerPrint');
  header.innerHTML = "<img src='images/logo.png'>";
  header.innerHTML += '<h3><br>' + document.querySelector('#linha').innerHTML + '</h3>';
  const tipo = document.querySelector('#tipoHorario').innerHTML;

  if (tipo == 'completo') {
    header.innerHTML += ' <p class="infoHeaderPrint">  HorÃ¡rio vÃ¡lido em: ' + document.querySelector('#dateInput').value + '</p><br><br>';
  } else {
    header.appendChild(document.querySelector('.infoRow'));
  }

  horario.appendChild(header);
  const tabela = document.createElement('div');
  tabela.innerHTML = document.querySelector('#paragens').innerHTML;
  horario.appendChild(tabela); // document.querySelector('body').appendChild(horario);

  window.print();
}

function verificaSeEnter(evento) {
  if (evento.key == 'Enter') {
    mostraLinha();
    document.querySelector('#inputLinha').blur();
  }
}

function offsetHorario() {
  var element = document.getElementById('horariosSeparador');
  var headerOffset = 40;
  var elementPosition = element.getBoundingClientRect().top;
  var offsetPosition = elementPosition + window.pageYOffset - headerOffset;
  window.scrollTo({
    top: offsetPosition,
    behavior: "smooth"
  });
}

function offsetId(id) {
  var element = document.getElementById(id);
  var headerOffset = 40;
  var elementPosition = element.getBoundingClientRect().top;
  var offsetPosition = elementPosition + window.pageYOffset - headerOffset;
  window.scrollTo({
    top: offsetPosition,
    behavior: "smooth"
  });
}
/*************************************************************************************/

/*** DOMContentLoaded ****************************************************************/

/*************************************************************************************/


document.addEventListener('DOMContentLoaded', () => {
  var [today, hora_agora] = mostraHoraEDiaNosInputs();
  /* insere a hora e o dia*/

  mostraSelectLinhas();
  /* insere o seletor de linhas */

  document.querySelector('#tipoHorario').innerHTML = 'meuHorario';
  mostraBotoes();

  if (!('listaLinhasPreferidas' in localStorage)) {
    // se nÃ£o existir lista de preferidos 
    document.querySelector('#separadorPreferidas').style.display = 'none';
  } else {
    // se existir lista de preferidos
    document.querySelector('#separadorPreferidas').style.display = 'block';
    document.querySelector('#separadorPreferidas').style.display = 'block';
    document.querySelector('#linhasPreferidasLista').style.display = 'block';
    document.querySelector('#linhasPreferidasContent').setAttribute('class', 'accordion__btn js-accordion-btn active');
    document.querySelector('#linhasPreferidasContent').parentElement.setAttribute('class', 'active');
    document.querySelector('#horariosContent').style.display = 'none';
    document.querySelector('#horariosSeparador').setAttribute('class', 'accordion__btn js-accordion-btn');
    document.querySelector('#horariosSeparador').parentElement.setAttribute('class', '');
  }
  /* eventlistener para o caso duma linha ser selecionada */


  document.querySelector('#inputLinha').onchange = mostraLinha;
});
const date_service_ids = {
  "20220101": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
  "20220102": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20220103": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220104": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220105": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220106": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220107": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220108": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220109": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
  "20220110": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220111": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220112": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220113": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220114": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220115": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220116": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220117": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220118": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220119": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220120": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220121": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220122": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220123": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220124": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220125": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220126": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220127": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220128": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220129": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220130": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220131": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220201": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220202": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220203": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220204": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220205": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220206": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20220207": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220208": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220209": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220210": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220211": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220212": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220213": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
  "20220214": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220215": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220216": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220217": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220218": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220219": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220220": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220221": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220222": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220223": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220224": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220225": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220226": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220227": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220228": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220301": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220302": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220303": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220304": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220305": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220306": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20220307": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220308": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220309": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220310": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220311": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220312": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220313": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
  "20220314": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220315": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220316": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220317": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220318": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220319": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220320": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220321": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220322": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220323": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220324": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220325": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220326": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220327": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220328": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220329": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220330": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220331": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220401": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220402": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220403": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20220404": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220405": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220406": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220407": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220408": ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20220409": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
  "20220410": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
  "20220411": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20220412": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20220413": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20220414": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20220415": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
  "20220416": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
  "20220417": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220418": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20220419": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20220420": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20220421": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20220422": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220423": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220424": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220425": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
  "20220426": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220427": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220428": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220429": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220430": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220501": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20220502": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220503": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220504": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220505": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220506": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220507": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220508": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
  "20220509": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220510": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220511": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220512": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220513": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220514": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220515": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220516": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220517": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220518": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220519": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220520": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220521": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220522": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220523": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220524": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220525": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220526": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220527": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220528": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220529": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220530": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220531": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220601": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220602": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220603": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220604": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220605": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20220606": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220607": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220608": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220609": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220610": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220611": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115", "118", "119", "120", "121"],
  "20220612": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220613": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
  "20220614": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
  "20220615": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
  "20220616": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220617": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
  "20220618": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115", "118", "119", "120", "121"],
  "20220619": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220620": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
  "20220621": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
  "20220622": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
  "20220623": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20220624": ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20220625": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220626": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220627": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20220628": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20220629": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20220630": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20220701": ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "121"],
  "20220702": ["1", "3", "7", "8", "11", "52", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220703": ["1", "2", "5", "8", "53", "54", "56", "97", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220704": ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220705": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220706": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220707": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220708": ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220709": ["1", "3", "7", "8", "11", "52", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220710": ["1", "2", "5", "8", "53", "54", "56", "69", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220711": ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220712": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220713": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220714": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220715": ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220716": ["1", "3", "7", "8", "11", "52", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220717": ["1", "2", "5", "8", "53", "54", "56", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220718": ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220719": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220720": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220721": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220722": ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220723": ["1", "3", "7", "8", "11", "52", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220724": ["1", "2", "5", "8", "53", "54", "56", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220725": ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220726": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220727": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220728": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220729": ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
  "20220730": ["1", "3", "7", "8", "11", "52", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220731": ["1", "2", "5", "8", "53", "54", "56", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220801": ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220802": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220803": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220804": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220805": ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220806": ["1", "3", "7", "8", "11", "52", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220807": ["1", "2", "5", "8", "53", "54", "56", "97", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220808": ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220809": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220810": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220811": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220812": ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220813": ["1", "3", "7", "8", "11", "52", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220814": ["1", "2", "5", "8", "53", "54", "56", "69", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220815": ["1", "2", "5", "8", "53", "54", "56", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220816": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220817": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220818": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220819": ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220820": ["1", "3", "7", "8", "11", "52", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220821": ["1", "2", "5", "8", "53", "54", "56", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220822": ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220823": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220824": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220825": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220826": ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220827": ["1", "3", "7", "8", "11", "52", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220828": ["1", "2", "5", "8", "53", "54", "56", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220829": ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220830": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220831": ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
  "20220901": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
  "20220902": ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
  "20220903": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220904": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220905": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
  "20220906": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
  "20220907": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
  "20220908": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
  "20220909": ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
  "20220910": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220911": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20220912": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
  "20220913": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
  "20220914": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
  "20220915": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
  "20220916": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220917": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220918": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220919": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220920": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220921": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220922": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220923": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220924": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20220925": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20220926": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220927": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220928": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220929": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20220930": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221001": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20221002": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20221003": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221004": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221005": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
  "20221006": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221007": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221008": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20221009": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
  "20221010": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221011": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221012": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221013": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221014": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221015": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20221016": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20221017": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221018": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221019": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221020": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221021": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221022": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20221023": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20221024": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221025": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221026": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221027": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221028": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221029": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20221030": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20221031": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221101": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
  "20221102": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221103": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221104": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221105": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20221106": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20221107": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221108": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221109": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221110": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221111": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221112": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20221113": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
  "20221114": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221115": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221116": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221117": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221118": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221119": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20221120": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20221121": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221122": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221123": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221124": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221125": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221126": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20221127": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20221128": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221129": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221130": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221201": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
  "20221202": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221203": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20221204": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20221205": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221206": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221207": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221208": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
  "20221209": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221210": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20221211": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
  "20221212": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221213": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221214": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221215": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221216": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20221217": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
  "20221218": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20221219": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20221220": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20221221": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20221222": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20221223": ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20221224": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
  "20221225": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20221226": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20221227": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20221228": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20221229": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20221230": ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20221231": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
  "20230101": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20230102": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20230103": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230104": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230105": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230106": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230107": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230108": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
  "20230109": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230110": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230111": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230112": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230113": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230114": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230115": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20230116": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230117": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230118": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230119": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230120": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230121": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230122": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20230123": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230124": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230125": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230126": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230127": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230128": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230129": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20230130": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230131": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230201": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230202": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230203": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230204": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230205": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20230206": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230207": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230208": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230209": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230210": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230211": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230212": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
  "20230213": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230214": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230215": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230216": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230217": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230218": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
  "20230219": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20230220": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20230221": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
  "20230222": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20230223": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230224": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230225": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230226": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20230227": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230228": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230301": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230302": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230303": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230304": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230305": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20230306": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230307": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230308": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230309": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230310": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230311": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230312": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
  "20230313": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230314": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230315": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230316": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230317": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230318": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230319": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20230320": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230321": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230322": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230323": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230324": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230325": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
  "20230326": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20230327": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20230328": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20230329": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20230330": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20230331": ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20230401": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
  "20230402": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20230403": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20230404": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20230405": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20230406": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
  "20230407": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
  "20230408": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
  "20230409": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
  "20230410": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230411": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230412": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230413": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230414": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230415": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230416": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20230417": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230418": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230419": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230420": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230421": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230422": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230423": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20230424": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230425": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
  "20230426": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230427": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230428": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230429": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230430": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20230501": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
  "20230502": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230503": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230504": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230505": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230506": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230507": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20230508": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230509": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230510": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230511": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230512": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230513": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230514": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
  "20230515": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230516": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230517": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230518": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230519": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230520": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230521": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20230522": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230523": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230524": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230525": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230526": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230527": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230528": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
  "20230529": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230530": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230531": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230601": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230602": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230603": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
  "20230604": ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
  "20230605": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230606": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230607": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230608": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
  "20230609": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
  "20230610": ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20230611": ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20230612": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
  "20230613": ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
  "20230614": ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
  "20230615": ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
  "20230616": ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
  "20230617": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115", "118", "119", "120", "121"],
  "20230618": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20230619": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20230620": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20230621": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20230622": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20230623": ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20230624": ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20230625": ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
  "20230626": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20230627": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20230628": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20230629": ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
  "20230630": ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"]
};
const designacao_linhas = {
  "4001": {
    "ida": "",
    "volta": "",
    "circular": "Alcochete | Circular",
    "ida_short": "",
    "volta_short": "",
    "circular_short": "Alcochete | Circ"
  },
  "4002": {
    "ida": "",
    "volta": "",
    "circular": "SÃ£o Francisco | Circular",
    "ida_short": "",
    "volta_short": "",
    "circular_short": "SÃ£o Francisco | "
  },
  "4103": {
    "ida": "<span>Moita (EstaÃ§Ã£o) </span><span>ğŸ –</span><span class='ultimo'> Sarilhos Pequenos</span>",
    "volta": "<span> Sarilhos Pequenos</span><span>ğŸ –</span><span class='ultimo'>Moita (EstaÃ§Ã£o) </span>",
    "circular": "",
    "ida_short": "<span>Moita (E</span><span>ğŸ –</span><span class='ultimo'> Sarilho</span>",
    "volta_short": "<span> Sarilho</span><span>ğŸ –</span><span class='ultimo'>Moita (E</span>",
    "circular_short": ""
  },
  "4101": {
    "ida": "<span>Alhos Vedros (Escola JosÃ© Afonso) </span><span>ğŸ –</span><span class='ultimo'> Arroteias</span>",
    "volta": "<span> Arroteias</span><span>ğŸ –</span><span class='ultimo'>Alhos Vedros (Escola JosÃ© Afonso) </span>",
    "circular": "",
    "ida_short": "<span>Alhos Ve</span><span>ğŸ –</span><span class='ultimo'> Arrotei</span>",
    "volta_short": "<span> Arrotei</span><span>ğŸ –</span><span class='ultimo'>Alhos Ve</span>",
    "circular_short": ""
  },
  "4104": {
    "ida": "",
    "volta": "",
    "circular": "Moita | Circular",
    "ida_short": "",
    "volta_short": "",
    "circular_short": "Moita | Circular"
  },
  "4205": {
    "ida": "<span>Bairro do CharqueirÃ£o </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal Fluvial), via Vale Porim</span>",
    "volta": "<span> Montijo (Terminal Fluvial), via Vale Porim</span><span>ğŸ –</span><span class='ultimo'>Bairro do CharqueirÃ£o </span>",
    "circular": "",
    "ida_short": "<span>Bairro d</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
    "volta_short": "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Bairro d</span>",
    "circular_short": ""
  },
  "4204": {
    "ida": "<span>Bairro do CharqueirÃ£o </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal Fluvial)</span>",
    "volta": "<span> Montijo (Terminal Fluvial)</span><span>ğŸ –</span><span class='ultimo'>Bairro do CharqueirÃ£o </span>",
    "circular": "",
    "ida_short": "<span>Bairro d</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
    "volta_short": "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Bairro d</span>",
    "circular_short": ""
  },
  "4206": {
    "ida": "<span>Bairro Esteval </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal Fluvial)</span>",
    "volta": "<span> Montijo (Terminal Fluvial)</span><span>ğŸ –</span><span class='ultimo'>Bairro Esteval </span>",
    "circular": "",
    "ida_short": "<span>Bairro E</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
    "volta_short": "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Bairro E</span>",
    "circular_short": ""
  },
  "4203": {
    "ida": "<span>Afonsoeiro </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal Fluvial), via Bairro da Liberdade</span>",
    "volta": "<span> Montijo (Terminal Fluvial), via Bairro da Liberdade</span><span>ğŸ –</span><span class='ultimo'>Afonsoeiro </span>",
    "circular": "",
    "ida_short": "<span>Afonsoei</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
    "volta_short": "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Afonsoei</span>",
    "circular_short": ""
  },
  "4202": {
    "ida": "<span>Afonsoeiro </span><span>ğŸ –</span><span class='ultimo'> Bairro do Saldanha, via Bairro da CalÃ§ada</span>",
    "volta": "<span> Bairro do Saldanha, via Bairro da CalÃ§ada</span><span>ğŸ –</span><span class='ultimo'>Afonsoeiro </span>",
    "circular": "",
    "ida_short": "<span>Afonsoei</span><span>ğŸ –</span><span class='ultimo'> Bairro </span>",
    "volta_short": "<span> Bairro </span><span>ğŸ –</span><span class='ultimo'>Afonsoei</span>",
    "circular_short": ""
  },
  "4208": {
    "ida": "<span>Montijo (Terminal RodoviÃ¡rio) </span><span>ğŸ –</span><span class='ultimo'> Sarilhos Grandes (Estr. 4 Marcos)</span>",
    "volta": "<span> Sarilhos Grandes (Estr. 4 Marcos)</span><span>ğŸ –</span><span class='ultimo'>Montijo (Terminal RodoviÃ¡rio) </span>",
    "circular": "",
    "ida_short": "<span>Montijo </span><span>ğŸ –</span><span class='ultimo'> Sarilho</span>",
    "volta_short": "<span> Sarilho</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
    "circular_short": ""
  },
  "4212": {
    "ida": "<span>Foros Boavista </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
    "volta": "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Foros Boavista </span>",
    "circular": "",
    "ida_short": "<span>Foros Bo</span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
    "volta_short": "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Foros Bo</span>",
    "circular_short": ""
  },
  "4210": {
    "ida": "<span>Canha </span><span>ğŸ –</span><span class='ultimo'> Foros Boavista</span>",
    "volta": "<span> Foros Boavista</span><span>ğŸ –</span><span class='ultimo'>Canha </span>",
    "circular": "",
    "ida_short": "<span>Canha </span><span>ğŸ –</span><span class='ultimo'> Foros B</span>",
    "volta_short": "<span> Foros B</span><span>ğŸ –</span><span class='ultimo'>Canha </span>",
    "circular_short": ""
  },
  "4201": {
    "ida": "<span>Afonsoeiro </span><span>ğŸ –</span><span class='ultimo'> Bairro da Liberdade</span>",
    "volta": "<span> Bairro da Liberdade</span><span>ğŸ –</span><span class='ultimo'>Afonsoeiro </span>",
    "circular": "",
    "ida_short": "<span>Afonsoei</span><span>ğŸ –</span><span class='ultimo'> Bairro </span>",
    "volta_short": "<span> Bairro </span><span>ğŸ –</span><span class='ultimo'>Afonsoei</span>",
    "circular_short": ""
  },
  "4211": {
    "ida": "",
    "volta": "",
    "circular": "Craveiras - PegÃµes | Circular",
    "ida_short": "",
    "volta_short": "",
    "circular_short": "Craveiras - PegÃµ"
  },
  "4207": {
    "ida": "<span>Montijo (Ã�rea Comercial) </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal Fluvial)</span>",
    "volta": "<span> Montijo (Terminal Fluvial)</span><span>ğŸ –</span><span class='ultimo'>Montijo (Ã�rea Comercial) </span>",
    "circular": "",
    "ida_short": "<span>Montijo </span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
    "volta_short": "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
    "circular_short": ""
  },
  "4303": {
    "ida": "",
    "volta": "",
    "circular": "Palmela | Circular",
    "ida_short": "",
    "volta_short": "",
    "circular_short": "Palmela | Circul"
  },
  "4305": {
    "ida": "<span>Brejos do Assa </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
    "volta": "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Brejos do Assa </span>",
    "circular": "",
    "ida_short": "<span>Brejos d</span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
    "volta_short": "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Brejos d</span>",
    "circular_short": ""
  },
  "4304": {
    "ida": "<span>Palmela (Terminal) </span><span>ğŸ –</span><span class='ultimo'> Penalva</span>",
    "volta": "<span> Penalva</span><span>ğŸ –</span><span class='ultimo'>Palmela (Terminal) </span>",
    "circular": "",
    "ida_short": "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> Penalva</span>",
    "volta_short": "<span> Penalva</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
    "circular_short": ""
  },
  "4322": {
    "ida": "<span>Pinhal Novo </span><span>ğŸ –</span><span class='ultimo'> Rio Frio</span>",
    "volta": "<span> Rio Frio</span><span>ğŸ –</span><span class='ultimo'>Pinhal Novo </span>",
    "circular": "",
    "ida_short": "<span>Pinhal N</span><span>ğŸ –</span><span class='ultimo'> Rio Fri</span>",
    "volta_short": "<span> Rio Fri</span><span>ğŸ –</span><span class='ultimo'>Pinhal N</span>",
    "circular_short": ""
  },
  "4307": {
    "ida": "<span>Loja Nova </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
    "volta": "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Loja Nova </span>",
    "circular": "",
    "ida_short": "<span>Loja Nov</span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
    "volta_short": "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Loja Nov</span>",
    "circular_short": ""
  },
  "4302": {
    "ida": "<span>Palmela (EstaÃ§Ã£o) </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
    "volta": "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Palmela (EstaÃ§Ã£o) </span>",
    "circular": "",
    "ida_short": "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
    "volta_short": "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
    "circular_short": ""
  },
  "4311": {
    "ida": "<span>Asseiceira </span><span>ğŸ –</span><span class='ultimo'> PoceirÃ£o</span>",
    "volta": "<span> PoceirÃ£o</span><span>ğŸ –</span><span class='ultimo'>Asseiceira </span>",
    "circular": "",
    "ida_short": "<span>Asseicei</span><span>ğŸ –</span><span class='ultimo'> PoceirÃ£</span>",
    "volta_short": "<span> PoceirÃ£</span><span>ğŸ –</span><span class='ultimo'>Asseicei</span>",
    "circular_short": ""
  },
  "4301": {
    "ida": "<span>Palmela (Centro) </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
    "volta": "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Palmela (Centro) </span>",
    "circular": "",
    "ida_short": "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
    "volta_short": "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
    "circular_short": ""
  },
  "4310": {
    "ida": "<span>Ã�guas de Moura </span><span>ğŸ –</span><span class='ultimo'> PoceirÃ£o</span>",
    "volta": "<span> PoceirÃ£o</span><span>ğŸ –</span><span class='ultimo'>Ã�guas de Moura </span>",
    "circular": "",
    "ida_short": "<span>Ã�guas de</span><span>ğŸ –</span><span class='ultimo'> PoceirÃ£</span>",
    "volta_short": "<span> PoceirÃ£</span><span>ğŸ –</span><span class='ultimo'>Ã�guas de</span>",
    "circular_short": ""
  },
  "4312": {
    "ida": "<span>PoceirÃ£o </span><span>ğŸ –</span><span class='ultimo'> Vale Abrunheira (X), via Fernando PÃ³</span>",
    "volta": "<span> Vale Abrunheira (X), via Fernando PÃ³</span><span>ğŸ –</span><span class='ultimo'>PoceirÃ£o </span>",
    "circular": "",
    "ida_short": "<span>PoceirÃ£o</span><span>ğŸ –</span><span class='ultimo'> Vale Ab</span>",
    "volta_short": "<span> Vale Ab</span><span>ğŸ –</span><span class='ultimo'>PoceirÃ£o</span>",
    "circular_short": ""
  },
  "4306": {
    "ida": "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
    "volta": "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
    "circular": "",
    "ida_short": "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
    "volta_short": "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
    "circular_short": ""
  },
  "4313": {
    "ida": "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Penalva</span>",
    "volta": "<span> Penalva</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
    "circular": "",
    "ida_short": "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Penalva</span>",
    "volta_short": "<span> Penalva</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
    "circular_short": ""
  },
  "4308": {
    "ida": "<span>Palmela (Terminal) </span><span>ğŸ –</span><span class='ultimo'> Pinhal Novo (EstaÃ§Ã£o)</span>",
    "volta": "<span> Pinhal Novo (EstaÃ§Ã£o)</span><span>ğŸ –</span><span class='ultimo'>Palmela (Terminal) </span>",
    "circular": "",
    "ida_short": "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> Pinhal </span>",
    "volta_short": "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
    "circular_short": ""
  },
  "4320": {
    "ida": "",
    "volta": "",
    "circular": "Pinhal Novo | Circular",
    "ida_short": "",
    "volta_short": "",
    "circular_short": "Pinhal Novo | Ci"
  },
  "4321": {
    "ida": "<span>Pinhal Novo </span><span>ğŸ –</span><span class='ultimo'> Qta do Anjo</span>",
    "volta": "<span> Qta do Anjo</span><span>ğŸ –</span><span class='ultimo'>Pinhal Novo </span>",
    "circular": "",
    "ida_short": "<span>Pinhal N</span><span>ğŸ –</span><span class='ultimo'> Qta do </span>",
    "volta_short": "<span> Qta do </span><span>ğŸ –</span><span class='ultimo'>Pinhal N</span>",
    "circular_short": ""
  },
  "4432": {
    "ida": "<span>SetÃºbal (ITS) </span><span>ğŸ –</span><span class='ultimo'> Vale de Choupo</span>",
    "volta": "<span> Vale de Choupo</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (ITS) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Vale de</span>",
    "volta_short": "<span> Vale de</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4412": {
    "ida": "<span>Morgada </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Mercado)</span>",
    "volta": "<span> SetÃºbal (Mercado)</span><span>ğŸ –</span><span class='ultimo'>Morgada </span>",
    "circular": "",
    "ida_short": "<span>Morgada </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Morgada </span>",
    "circular_short": ""
  },
  "4413": {
    "ida": "",
    "volta": "<span> SetÃºbal (Mercado), via Bela Vista</span><span>ğŸ –</span><span class='ultimo'>Morgada </span>",
    "circular": "",
    "ida_short": "",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Morgada </span>",
    "circular_short": ""
  },
  "4406": {
    "ida": "<span>Manteigadas </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Mercado)</span>",
    "volta": "<span> SetÃºbal (Mercado)</span><span>ğŸ –</span><span class='ultimo'>Manteigadas </span>",
    "circular": "",
    "ida_short": "<span>Manteiga</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Manteiga</span>",
    "circular_short": ""
  },
  "4408": {
    "ida": "",
    "volta": "<span> SetÃºbal (Mercado), via Bela Vista</span><span>ğŸ –</span><span class='ultimo'>Manteigadas </span>",
    "circular": "",
    "ida_short": "",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Manteiga</span>",
    "circular_short": ""
  },
  "4436": {
    "ida": "<span>SetÃºbal (Mercado) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Av. Soeiro Pereira Gomes)</span>",
    "volta": "<span> SetÃºbal (Av. Soeiro Pereira Gomes)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Mercado) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4420": {
    "ida": "<span>SetÃºbal (Alegro) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Alegro) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4416": {
    "ida": "<span>PoÃ§o Mouro </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>PoÃ§o Mouro </span>",
    "circular": "",
    "ida_short": "<span>PoÃ§o Mou</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>PoÃ§o Mou</span>",
    "circular_short": ""
  },
  "4431": {
    "ida": "<span>SetÃºbal (ITS) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Quinta Varzinha)</span>",
    "volta": "<span> SetÃºbal (Quinta Varzinha)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (ITS) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4434": {
    "ida": "<span>SetÃºbal (Mercado 2 de Abril) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (R. Timor)</span>",
    "volta": "<span> SetÃºbal (R. Timor)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Mercado 2 de Abril) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4426": {
    "ida": "<span>SetÃºbal (Bairro Viso) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (CHEsetÃºbal)</span>",
    "volta": "<span> SetÃºbal (CHEsetÃºbal)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Bairro Viso) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4425": {
    "ida": "<span>SetÃºbal (Escola Viso) </span><span>ğŸ –</span><span class='ultimo'> Mitrena</span>",
    "volta": "<span> Mitrena</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Escola Viso) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Mitrena</span>",
    "volta_short": "<span> Mitrena</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4424": {
    "ida": "<span>SetÃºbal (Bairro Viso) </span><span>ğŸ –</span><span class='ultimo'> Manteigadas</span>",
    "volta": "<span> Manteigadas</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Bairro Viso) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Manteig</span>",
    "volta_short": "<span> Manteig</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4422": {
    "ida": "<span>SetÃºbal (Bairro Camolas) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Casal Figueiras), via Bairro do Viso</span>",
    "volta": "<span> SetÃºbal (Casal Figueiras), via Bairro do Viso</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Bairro Camolas) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4428": {
    "ida": "<span>SetÃºbal (Casal Figueiras) </span><span>ğŸ –</span><span class='ultimo'> Vale Ana Gomes</span>",
    "volta": "<span> Vale Ana Gomes</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Casal Figueiras) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Vale An</span>",
    "volta_short": "<span> Vale An</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4421": {
    "ida": "<span>SetÃºbal (Bairro Camolas) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Casal Figueiras)</span>",
    "volta": "<span> SetÃºbal (Casal Figueiras)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Bairro Camolas) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4440": {
    "ida": "<span>SetÃºbal (Monte Belo Norte) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Saboaria), via Alegro</span>",
    "volta": "<span> SetÃºbal (Saboaria), via Alegro</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Monte Belo Norte) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4438": {
    "ida": "<span>SetÃºbal (Monte Belo Norte) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Saboaria)</span>",
    "volta": "<span> SetÃºbal (Saboaria)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Monte Belo Norte) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4419": {
    "ida": "<span>Brejos Canes </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Saboaria)</span>",
    "volta": "<span> SetÃºbal (Saboaria)</span><span>ğŸ –</span><span class='ultimo'>Brejos Canes </span>",
    "circular": "",
    "ida_short": "<span>Brejos C</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Brejos C</span>",
    "circular_short": ""
  },
  "4441": {
    "ida": "<span>SetÃºbal (Saboaria) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Vale Cobro)</span>",
    "volta": "<span> SetÃºbal (Vale Cobro)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Saboaria) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4433": {
    "ida": "<span>Alto Guerra </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Casal Figueiras)</span>",
    "volta": "<span> SetÃºbal (Casal Figueiras)</span><span>ğŸ –</span><span class='ultimo'>Alto Guerra </span>",
    "circular": "",
    "ida_short": "<span>Alto Gue</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Alto Gue</span>",
    "circular_short": ""
  },
  "4407": {
    "ida": "<span>Manteigadas </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Mercado), via Bairro da Carmona</span>",
    "volta": "",
    "circular": "",
    "ida_short": "<span>Manteiga</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "",
    "circular_short": ""
  },
  "4472": {
    "ida": "<span>Praia do Creiro </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Praia do Creiro </span>",
    "circular": "",
    "ida_short": "<span>Praia do</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Praia do</span>",
    "circular_short": ""
  },
  "4474": {
    "ida": "<span>Figueirinha </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Alegro)</span>",
    "volta": "<span> SetÃºbal (Alegro)</span><span>ğŸ –</span><span class='ultimo'>Figueirinha </span>",
    "circular": "",
    "ida_short": "<span>Figueiri</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Figueiri</span>",
    "circular_short": ""
  },
  "4471": {
    "ida": "",
    "volta": "",
    "circular": "Praia Albarquel | Circular",
    "ida_short": "",
    "volta_short": "",
    "circular_short": "Praia Albarquel "
  },
  "4470": {
    "ida": "<span>Brejos AzeitÃ£o </span><span>ğŸ –</span><span class='ultimo'> Praia do Creiro</span>",
    "volta": "<span> Praia do Creiro</span><span>ğŸ –</span><span class='ultimo'>Brejos AzeitÃ£o </span>",
    "circular": "",
    "ida_short": "<span>Brejos A</span><span>ğŸ –</span><span class='ultimo'> Praia d</span>",
    "volta_short": "<span> Praia d</span><span>ğŸ –</span><span class='ultimo'>Brejos A</span>",
    "circular_short": ""
  },
  "4414": {
    "ida": "<span>OutÃ£o (Hospital) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>OutÃ£o (Hospital) </span>",
    "circular": "",
    "ida_short": "<span>OutÃ£o (H</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>OutÃ£o (H</span>",
    "circular_short": ""
  },
  "4415": {
    "ida": "<span>OutÃ£o (Hospital) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS), via vale da Rasca</span>",
    "volta": "<span> SetÃºbal (ITS), via vale da Rasca</span><span>ğŸ –</span><span class='ultimo'>OutÃ£o (Hospital) </span>",
    "circular": "",
    "ida_short": "<span>OutÃ£o (H</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>OutÃ£o (H</span>",
    "circular_short": ""
  },
  "4452": {
    "ida": "<span>Mitrena (Portucel) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Mitrena (Portucel) </span>",
    "circular": "",
    "ida_short": "<span>Mitrena </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Mitrena </span>",
    "circular_short": ""
  },
  "4453": {
    "ida": "<span>Mitrena (Portucel) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS), via Estrada GraÃ§a</span>",
    "volta": "<span> SetÃºbal (ITS), via Estrada GraÃ§a</span><span>ğŸ –</span><span class='ultimo'>Mitrena (Portucel) </span>",
    "circular": "",
    "ida_short": "<span>Mitrena </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Mitrena </span>",
    "circular_short": ""
  },
  "4442": {
    "ida": "<span>Praias do Sado (EstaÃ§Ã£o) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Bela Vista)</span>",
    "volta": "<span> SetÃºbal (Bela Vista)</span><span>ğŸ –</span><span class='ultimo'>Praias do Sado (EstaÃ§Ã£o) </span>",
    "circular": "",
    "ida_short": "<span>Praias d</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Praias d</span>",
    "circular_short": ""
  },
  "4451": {
    "ida": "<span>Mitrena (Lisnave) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Mitrena (Lisnave) </span>",
    "circular": "",
    "ida_short": "<span>Mitrena </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Mitrena </span>",
    "circular_short": ""
  },
  "4402": {
    "ida": "<span>Estefanilha </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Estefanilha </span>",
    "circular": "",
    "ida_short": "<span>Estefani</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Estefani</span>",
    "circular_short": ""
  },
  "4437": {
    "ida": "<span>FaralhÃ£o </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>FaralhÃ£o </span>",
    "circular": "",
    "ida_short": "<span>FaralhÃ£o</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>FaralhÃ£o</span>",
    "circular_short": ""
  },
  "4411": {
    "ida": "<span>Morgada </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Morgada </span>",
    "circular": "",
    "ida_short": "<span>Morgada </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Morgada </span>",
    "circular_short": ""
  },
  "4439": {
    "ida": "<span>Praias do Sado </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Praias do Sado </span>",
    "circular": "",
    "ida_short": "<span>Praias d</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Praias d</span>",
    "circular_short": ""
  },
  "4460": {
    "ida": "AzeitÃ£o | Circular",
    "volta": "AzeitÃ£o | Circular",
    "circular": "",
    "ida_short": "",
    "volta_short": "AzeitÃ£o | Circul",
    "circular_short": ""
  },
  "4435": {
    "ida": "<span>Biscainho </span><span>ğŸ –</span><span class='ultimo'> FaralhÃ£o</span>",
    "volta": "<span> FaralhÃ£o</span><span>ğŸ –</span><span class='ultimo'>Biscainho </span>",
    "circular": "",
    "ida_short": "<span>Biscainh</span><span>ğŸ –</span><span class='ultimo'> FaralhÃ£</span>",
    "volta_short": "<span> FaralhÃ£</span><span>ğŸ –</span><span class='ultimo'>Biscainh</span>",
    "circular_short": ""
  },
  "4429": {
    "ida": "<span>SetÃºbal (Centro SaÃºde) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Mercado)</span>",
    "volta": "<span> SetÃºbal (Mercado)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Centro SaÃºde) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4410": {
    "ida": "<span>Manteigadas (Esc. Profissional) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Alegro)</span>",
    "volta": "<span> SetÃºbal (Alegro)</span><span>ğŸ –</span><span class='ultimo'>Manteigadas (Esc. Profissional) </span>",
    "circular": "",
    "ida_short": "<span>Manteiga</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Manteiga</span>",
    "circular_short": ""
  },
  "4404": {
    "ida": "",
    "volta": "",
    "circular": "Interfaces SetÃºbal | Circular",
    "ida_short": "",
    "volta_short": "",
    "circular_short": "Interfaces SetÃºb"
  },
  "4409": {
    "ida": "<span>Manteigadas </span><span>ğŸ –</span><span class='ultimo'> Viso</span>",
    "volta": "<span> Viso</span><span>ğŸ –</span><span class='ultimo'>Manteigadas </span>",
    "circular": "",
    "ida_short": "<span>Manteiga</span><span>ğŸ –</span><span class='ultimo'> Viso</span>",
    "volta_short": "<span> Viso</span><span>ğŸ –</span><span class='ultimo'>Manteiga</span>",
    "circular_short": ""
  },
  "4427": {
    "ida": "<span>SetÃºbal (Bela Vista) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Mercado)</span>",
    "volta": "<span> SetÃºbal (Mercado)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Bela Vista) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4430": {
    "ida": "<span>SetÃºbal (Hospital) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (MontalvÃ£o)</span>",
    "volta": "<span> SetÃºbal (MontalvÃ£o)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Hospital) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4401": {
    "ida": "<span>Cachofarra </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Hospital)</span>",
    "volta": "<span> SetÃºbal (Hospital)</span><span>ğŸ –</span><span class='ultimo'>Cachofarra </span>",
    "circular": "",
    "ida_short": "<span>Cachofar</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Cachofar</span>",
    "circular_short": ""
  },
  "4443": {
    "ida": "<span>SetÃºbal (PolitÃ©cnico) </span><span>ğŸ –</span><span class='ultimo'> Praias do Sado</span>",
    "volta": "<span> Praias do Sado</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (PolitÃ©cnico) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Praias </span>",
    "volta_short": "<span> Praias </span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4475": {
    "ida": "<span>Portinho da ArrÃ¡bida </span><span>ğŸ –</span><span class='ultimo'> Viso</span>",
    "volta": "<span> Viso</span><span>ğŸ –</span><span class='ultimo'>Portinho da ArrÃ¡bida </span>",
    "circular": "",
    "ida_short": "<span>Portinho</span><span>ğŸ –</span><span class='ultimo'> Viso</span>",
    "volta_short": "<span> Viso</span><span>ğŸ –</span><span class='ultimo'>Portinho</span>",
    "circular_short": ""
  },
  "4417": {
    "ida": "<span>PoÃ§o Mouro </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS), via Manteigadas</span>",
    "volta": "<span> SetÃºbal (ITS), via Manteigadas</span><span>ğŸ –</span><span class='ultimo'>PoÃ§o Mouro </span>",
    "circular": "",
    "ida_short": "<span>PoÃ§o Mou</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>PoÃ§o Mou</span>",
    "circular_short": ""
  },
  "4418": {
    "ida": "<span>SetÃºbal (Alegro) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Av. 5 Outubro)</span>",
    "volta": "<span> SetÃºbal (Av. 5 Outubro)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Alegro) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4403": {
    "ida": "<span>Fonte da Talha </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Av. LuÃ­sa Todi)</span>",
    "volta": "<span> SetÃºbal (Av. LuÃ­sa Todi)</span><span>ğŸ –</span><span class='ultimo'>Fonte da Talha </span>",
    "circular": "",
    "ida_short": "<span>Fonte da</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Fonte da</span>",
    "circular_short": ""
  },
  "4423": {
    "ida": "<span>Amoreiras </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Av. LuÃ­sa Todi)</span>",
    "volta": "<span> SetÃºbal (Av. LuÃ­sa Todi)</span><span>ğŸ –</span><span class='ultimo'>Amoreiras </span>",
    "circular": "",
    "ida_short": "<span>Amoreira</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Amoreira</span>",
    "circular_short": ""
  },
  "4405": {
    "ida": "Livramento-Montebelo | Circular",
    "volta": "Livramento-Montebelo | Circular",
    "circular": "",
    "ida_short": "",
    "volta_short": "Livramento-Monte",
    "circular_short": ""
  },
  "4476": {
    "ida": "",
    "volta": "",
    "circular": "Praias ArrÃ¡bida | Circular",
    "ida_short": "",
    "volta_short": "",
    "circular_short": "Praias ArrÃ¡bida "
  },
  "4561": {
    "ida": "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Vila Nogueira de AzeitÃ£o, via Quinta do PicÃ£o</span>",
    "volta": "<span> Vila Nogueira de AzeitÃ£o, via Quinta do PicÃ£o</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
    "circular": "",
    "ida_short": "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Vila No</span>",
    "volta_short": "<span> Vila No</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
    "circular_short": ""
  },
  "4560": {
    "ida": "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Vila Nogueira de AzeitÃ£o</span>",
    "volta": "<span> Vila Nogueira de AzeitÃ£o</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
    "circular": "",
    "ida_short": "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Vila No</span>",
    "volta_short": "<span> Vila No</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
    "circular_short": ""
  },
  "4532": {
    "ida": "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Quatro Marcos</span>",
    "volta": "<span> Quatro Marcos</span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
    "circular": "",
    "ida_short": "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Quatro </span>",
    "volta_short": "<span> Quatro </span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
    "circular_short": ""
  },
  "4531": {
    "ida": "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
    "volta": "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
    "circular": "",
    "ida_short": "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
    "volta_short": "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
    "circular_short": ""
  },
  "4503": {
    "ida": "<span>Atalaia </span><span>ğŸ –</span><span class='ultimo'> Jardia</span>",
    "volta": "<span> Jardia</span><span>ğŸ –</span><span class='ultimo'>Atalaia </span>",
    "circular": "",
    "ida_short": "<span>Atalaia </span><span>ğŸ –</span><span class='ultimo'> Jardia</span>",
    "volta_short": "<span> Jardia</span><span>ğŸ –</span><span class='ultimo'>Atalaia </span>",
    "circular_short": ""
  },
  "4504": {
    "ida": "<span>Montijo (Terminal Fluvial) </span><span>ğŸ –</span><span class='ultimo'> Passil</span>",
    "volta": "<span> Passil</span><span>ğŸ –</span><span class='ultimo'>Montijo (Terminal Fluvial) </span>",
    "circular": "",
    "ida_short": "<span>Montijo </span><span>ğŸ –</span><span class='ultimo'> Passil</span>",
    "volta_short": "<span> Passil</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
    "circular_short": ""
  },
  "4551": {
    "ida": "<span>Palmela (USF) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Av. LuÃ­sa Todi)</span>",
    "volta": "<span> SetÃºbal (Av. LuÃ­sa Todi)</span><span>ğŸ –</span><span class='ultimo'>Palmela (USF) </span>",
    "circular": "",
    "ida_short": "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
    "circular_short": ""
  },
  "4501": {
    "ida": "<span>Alcochete </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal Fluvial)</span>",
    "volta": "<span> Montijo (Terminal Fluvial)</span><span>ğŸ –</span><span class='ultimo'>Alcochete </span>",
    "circular": "",
    "ida_short": "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
    "volta_short": "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
    "circular_short": ""
  },
  "4512": {
    "ida": "<span>Alcochete (Freeport) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS), via Alto Estanqueiro</span>",
    "volta": "<span> SetÃºbal (ITS), via Alto Estanqueiro</span><span>ğŸ –</span><span class='ultimo'>Alcochete (Freeport) </span>",
    "circular": "",
    "ida_short": "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
    "circular_short": ""
  },
  "4513": {
    "ida": "<span>Alcochete (Freeport) </span><span>ğŸ –</span><span class='ultimo'> Pinhal Novo</span>",
    "volta": "<span> Pinhal Novo</span><span>ğŸ –</span><span class='ultimo'>Alcochete (Freeport) </span>",
    "circular": "",
    "ida_short": "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> Pinhal </span>",
    "volta_short": "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
    "circular_short": ""
  },
  "4510": {
    "ida": "<span>Alcochete (Freeport) </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal RodoviÃ¡rio)</span>",
    "volta": "<span> Montijo (Terminal RodoviÃ¡rio)</span><span>ğŸ –</span><span class='ultimo'>Alcochete (Freeport) </span>",
    "circular": "",
    "ida_short": "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
    "volta_short": "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
    "circular_short": ""
  },
  "4523": {
    "ida": "",
    "volta": "<span> Pinhal Novo</span><span>ğŸ –</span><span class='ultimo'>Montijo (Terminal RodoviÃ¡rio) </span>",
    "circular": "",
    "ida_short": "",
    "volta_short": "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
    "circular_short": ""
  },
  "4517": {
    "ida": "",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Montijo (Terminal RodoviÃ¡rio) </span>",
    "circular": "",
    "ida_short": "",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
    "circular_short": ""
  },
  "4514": {
    "ida": "<span>Canha </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal RodoviÃ¡rio), via PegÃµes</span>",
    "volta": "<span> Montijo (Terminal RodoviÃ¡rio), via PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Canha </span>",
    "circular": "",
    "ida_short": "<span>Canha </span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
    "volta_short": "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Canha </span>",
    "circular_short": ""
  },
  "4515": {
    "ida": "<span>Montijo (Terminal RodoviÃ¡rio) </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
    "volta": "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Montijo (Terminal RodoviÃ¡rio) </span>",
    "circular": "",
    "ida_short": "<span>Montijo </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
    "volta_short": "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
    "circular_short": ""
  },
  "4502": {
    "ida": "<span>Alcochete </span><span>ğŸ –</span><span class='ultimo'> Passil</span>",
    "volta": "<span> Passil</span><span>ğŸ –</span><span class='ultimo'>Alcochete </span>",
    "circular": "",
    "ida_short": "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> Passil</span>",
    "volta_short": "<span> Passil</span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
    "circular_short": ""
  },
  "4516": {
    "ida": "<span>Montijo (Terminal RodoviÃ¡rio) </span><span>ğŸ –</span><span class='ultimo'> Rio Frio</span>",
    "volta": "<span> Rio Frio</span><span>ğŸ –</span><span class='ultimo'>Montijo (Terminal RodoviÃ¡rio) </span>",
    "circular": "",
    "ida_short": "<span>Montijo </span><span>ğŸ –</span><span class='ultimo'> Rio Fri</span>",
    "volta_short": "<span> Rio Fri</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
    "circular_short": ""
  },
  "4520": {
    "ida": "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
    "volta": "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
    "circular": "",
    "ida_short": "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
    "volta_short": "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
    "circular_short": ""
  },
  "4521": {
    "ida": "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> Pinhal Novo</span>",
    "volta": "<span> Pinhal Novo</span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
    "circular": "",
    "ida_short": "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> Pinhal </span>",
    "volta_short": "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
    "circular_short": ""
  },
  "4522": {
    "ida": "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> PoceirÃ£o</span>",
    "volta": "<span> PoceirÃ£o</span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
    "circular": "",
    "ida_short": "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> PoceirÃ£</span>",
    "volta_short": "<span> PoceirÃ£</span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
    "circular_short": ""
  },
  "4524": {
    "ida": "<span>Palmela (Terminal) </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
    "volta": "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Palmela (Terminal) </span>",
    "circular": "",
    "ida_short": "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
    "volta_short": "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
    "circular_short": ""
  },
  "4530": {
    "ida": "<span>Bairro Vila Morena </span><span>ğŸ –</span><span class='ultimo'> Pinhal Novo</span>",
    "volta": "<span> Pinhal Novo</span><span>ğŸ –</span><span class='ultimo'>Bairro Vila Morena </span>",
    "circular": "",
    "ida_short": "<span>Bairro V</span><span>ğŸ –</span><span class='ultimo'> Pinhal </span>",
    "volta_short": "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Bairro V</span>",
    "circular_short": ""
  },
  "4541": {
    "ida": "<span>Algeruz </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Av. LuÃ­sa Todi)</span>",
    "volta": "<span> SetÃºbal (Av. LuÃ­sa Todi)</span><span>ğŸ –</span><span class='ultimo'>Algeruz </span>",
    "circular": "",
    "ida_short": "<span>Algeruz </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Algeruz </span>",
    "circular_short": ""
  },
  "4545": {
    "ida": "<span>Biscainho </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Bela Vista)</span>",
    "volta": "<span> SetÃºbal (Bela Vista)</span><span>ğŸ –</span><span class='ultimo'>Biscainho </span>",
    "circular": "",
    "ida_short": "<span>Biscainh</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Biscainh</span>",
    "circular_short": ""
  },
  "4548": {
    "ida": "<span>LagameÃ§as </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>LagameÃ§as </span>",
    "circular": "",
    "ida_short": "<span>LagameÃ§a</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>LagameÃ§a</span>",
    "circular_short": ""
  },
  "4544": {
    "ida": "<span>Bairro MargaÃ§a </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Bairro MargaÃ§a </span>",
    "circular": "",
    "ida_short": "<span>Bairro M</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Bairro M</span>",
    "circular_short": ""
  },
  "4540": {
    "ida": "<span>Ã�guas de Moura </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Ã�guas de Moura </span>",
    "circular": "",
    "ida_short": "<span>Ã�guas de</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Ã�guas de</span>",
    "circular_short": ""
  },
  "4547": {
    "ida": "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
    "circular": "",
    "ida_short": "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
    "circular_short": ""
  },
  "4562": {
    "ida": "<span>SetÃºbal (ITS) </span><span>ğŸ –</span><span class='ultimo'> Vila Nogueira de AzeitÃ£o, via Palmela (EstaÃ§Ã£o)</span>",
    "volta": "<span> Vila Nogueira de AzeitÃ£o, via Palmela (EstaÃ§Ã£o)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (ITS) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Vila No</span>",
    "volta_short": "<span> Vila No</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  },
  "4549": {
    "ida": "<span>Palmela (Terminal) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Palmela (Terminal) </span>",
    "circular": "",
    "ida_short": "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
    "circular_short": ""
  },
  "4550": {
    "ida": "<span>Palmela (Terminal) </span><span>ğŸ –</span><span class='ultimo'> Vila Nogueira de AzeitÃ£o</span>",
    "volta": "<span> Vila Nogueira de AzeitÃ£o</span><span>ğŸ –</span><span class='ultimo'>Palmela (Terminal) </span>",
    "circular": "",
    "ida_short": "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> Vila No</span>",
    "volta_short": "<span> Vila No</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
    "circular_short": ""
  },
  "4546": {
    "ida": "<span>Biscainho </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Biscainho </span>",
    "circular": "",
    "ida_short": "<span>Biscainh</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Biscainh</span>",
    "circular_short": ""
  },
  "4542": {
    "ida": "<span>Algeruz </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Algeruz </span>",
    "circular": "",
    "ida_short": "<span>Algeruz </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Algeruz </span>",
    "circular_short": ""
  },
  "4543": {
    "ida": "<span>Algeruz </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS), via PoÃ§oilos</span>",
    "volta": "<span> SetÃºbal (ITS), via PoÃ§oilos</span><span>ğŸ –</span><span class='ultimo'>Algeruz </span>",
    "circular": "",
    "ida_short": "<span>Algeruz </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Algeruz </span>",
    "circular_short": ""
  },
  "4511": {
    "ida": "<span>Alcochete (Freeport) </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal RodoviÃ¡rio), via Samouco</span>",
    "volta": "<span> Montijo (Terminal RodoviÃ¡rio), via Samouco</span><span>ğŸ –</span><span class='ultimo'>Alcochete (Freeport) </span>",
    "circular": "",
    "ida_short": "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
    "volta_short": "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
    "circular_short": ""
  },
  "4642": {
    "ida": "<span>Sesimbra (Terminal) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Hospital)</span>",
    "volta": "<span> SetÃºbal (Hospital)</span><span>ğŸ –</span><span class='ultimo'>Sesimbra (Terminal) </span>",
    "circular": "",
    "ida_short": "<span>Sesimbra</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Sesimbra</span>",
    "circular_short": ""
  },
  "4621": {
    "ida": "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Seixal (Terminal Fluvial)</span>",
    "volta": "<span> Seixal (Terminal Fluvial)</span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
    "circular": "",
    "ida_short": "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Seixal </span>",
    "volta_short": "<span> Seixal </span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
    "circular_short": ""
  },
  "4640": {
    "ida": "<span>Casais da Serra </span><span>ğŸ –</span><span class='ultimo'> Vila Nogueira de AzeitÃ£o</span>",
    "volta": "<span> Vila Nogueira de AzeitÃ£o</span><span>ğŸ –</span><span class='ultimo'>Casais da Serra </span>",
    "circular": "",
    "ida_short": "<span>Casais d</span><span>ğŸ –</span><span class='ultimo'> Vila No</span>",
    "volta_short": "<span> Vila No</span><span>ğŸ –</span><span class='ultimo'>Casais d</span>",
    "circular_short": ""
  },
  "4620": {
    "ida": "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Paio Pires</span>",
    "volta": "<span> Paio Pires</span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
    "circular": "",
    "ida_short": "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Paio Pi</span>",
    "volta_short": "<span> Paio Pi</span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
    "circular_short": ""
  },
  "4643": {
    "ida": "<span>Montijo (Av. Inf. D. Henrique) </span><span>ğŸ –</span><span class='ultimo'> Sesimbra (Terminal)</span>",
    "volta": "<span> Sesimbra (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Montijo (Av. Inf. D. Henrique) </span>",
    "circular": "",
    "ida_short": "<span>Montijo </span><span>ğŸ –</span><span class='ultimo'> Sesimbr</span>",
    "volta_short": "<span> Sesimbr</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
    "circular_short": ""
  },
  "4631": {
    "ida": "<span>Fogueteiro (EstaÃ§Ã£o) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Fogueteiro (EstaÃ§Ã£o) </span>",
    "circular": "",
    "ida_short": "<span>Foguetei</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Foguetei</span>",
    "circular_short": ""
  },
  "4630": {
    "ida": "<span>Corroios (EstaÃ§Ã£o) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Corroios (EstaÃ§Ã£o) </span>",
    "circular": "",
    "ida_short": "<span>Corroios</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Corroios</span>",
    "circular_short": ""
  },
  "4641": {
    "ida": "<span>Quinta do Conde </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Quinta do Conde </span>",
    "circular": "",
    "ida_short": "<span>Quinta d</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Quinta d</span>",
    "circular_short": ""
  },
  "4610": {
    "ida": "<span>Bairro dos Marinheiros </span><span>ğŸ –</span><span class='ultimo'> Barreiro (Terminal)</span>",
    "volta": "<span> Barreiro (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Bairro dos Marinheiros </span>",
    "circular": "",
    "ida_short": "<span>Bairro d</span><span>ğŸ –</span><span class='ultimo'> Barreir</span>",
    "volta_short": "<span> Barreir</span><span>ğŸ –</span><span class='ultimo'>Bairro d</span>",
    "circular_short": ""
  },
  "4602": {
    "ida": "<span>Alhos Vedros (EstaÃ§Ã£o) </span><span>ğŸ –</span><span class='ultimo'> Barreiro (Terminal)</span>",
    "volta": "<span> Barreiro (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Alhos Vedros (EstaÃ§Ã£o) </span>",
    "circular": "",
    "ida_short": "<span>Alhos Ve</span><span>ğŸ –</span><span class='ultimo'> Barreir</span>",
    "volta_short": "<span> Barreir</span><span>ğŸ –</span><span class='ultimo'>Alhos Ve</span>",
    "circular_short": ""
  },
  "4612": {
    "ida": "<span>Bairro dos Marinheiros </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
    "volta": "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Bairro dos Marinheiros </span>",
    "circular": "",
    "ida_short": "<span>Bairro d</span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
    "volta_short": "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Bairro d</span>",
    "circular_short": ""
  },
  "4605": {
    "ida": "<span>Lavradio </span><span>ğŸ –</span><span class='ultimo'> Pinhal do Forno</span>",
    "volta": "<span> Pinhal do Forno</span><span>ğŸ –</span><span class='ultimo'>Lavradio </span>",
    "circular": "",
    "ida_short": "<span>Lavradio</span><span>ğŸ –</span><span class='ultimo'> Pinhal </span>",
    "volta_short": "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Lavradio</span>",
    "circular_short": ""
  },
  "4600": {
    "ida": "<span>Alcochete (Freeport) </span><span>ğŸ –</span><span class='ultimo'> Barreiro (Terminal)</span>",
    "volta": "<span> Barreiro (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Alcochete (Freeport) </span>",
    "circular": "",
    "ida_short": "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> Barreir</span>",
    "volta_short": "<span> Barreir</span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
    "circular_short": ""
  },
  "4604": {
    "ida": "<span>Barreiro (Terminal) </span><span>ğŸ –</span><span class='ultimo'> Moita (Escola Fragata do Tejo)</span>",
    "volta": "<span> Moita (Escola Fragata do Tejo)</span><span>ğŸ –</span><span class='ultimo'>Barreiro (Terminal) </span>",
    "circular": "",
    "ida_short": "<span>Barreiro</span><span>ğŸ –</span><span class='ultimo'> Moita (</span>",
    "volta_short": "<span> Moita (</span><span>ğŸ –</span><span class='ultimo'>Barreiro</span>",
    "circular_short": ""
  },
  "4611": {
    "ida": "<span>Penalva </span><span>ğŸ –</span><span class='ultimo'> Moita (Esc. SecundÃ¡ria)</span>",
    "volta": "<span> Moita (Esc. SecundÃ¡ria)</span><span>ğŸ –</span><span class='ultimo'>Penalva </span>",
    "circular": "",
    "ida_short": "<span>Penalva </span><span>ğŸ –</span><span class='ultimo'> Moita (</span>",
    "volta_short": "<span> Moita (</span><span>ğŸ –</span><span class='ultimo'>Penalva </span>",
    "circular_short": ""
  },
  "4102": {
    "ida": "<span>CabeÃ§o Verde </span><span>ğŸ –</span><span class='ultimo'> Sarilhos Pequenos</span>",
    "volta": "<span> Sarilhos Pequenos</span><span>ğŸ –</span><span class='ultimo'>CabeÃ§o Verde </span>",
    "circular": "",
    "ida_short": "<span>CabeÃ§o V</span><span>ğŸ –</span><span class='ultimo'> Sarilho</span>",
    "volta_short": "<span> Sarilho</span><span>ğŸ –</span><span class='ultimo'>CabeÃ§o V</span>",
    "circular_short": ""
  },
  "4603": {
    "ida": "<span>Barreiro (Terminal) </span><span>ğŸ –</span><span class='ultimo'> ChÃ£o Duro</span>",
    "volta": "<span> ChÃ£o Duro</span><span>ğŸ –</span><span class='ultimo'>Barreiro (Terminal) </span>",
    "circular": "",
    "ida_short": "<span>Barreiro</span><span>ğŸ –</span><span class='ultimo'> ChÃ£o Du</span>",
    "volta_short": "<span> ChÃ£o Du</span><span>ğŸ –</span><span class='ultimo'>Barreiro</span>",
    "circular_short": ""
  },
  "4601": {
    "ida": "<span>Barreiro (Terminal) </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal RodoviÃ¡rio)</span>",
    "volta": "<span> Montijo (Terminal RodoviÃ¡rio)</span><span>ğŸ –</span><span class='ultimo'>Barreiro (Terminal) </span>",
    "circular": "",
    "ida_short": "<span>Barreiro</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
    "volta_short": "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Barreiro</span>",
    "circular_short": ""
  },
  "4701": {
    "ida": "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> Vale da Amoreira</span>",
    "volta": "<span> Vale da Amoreira</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
    "circular": "",
    "ida_short": "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> Vale da</span>",
    "volta_short": "<span> Vale da</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
    "circular_short": ""
  },
  "4703": {
    "ida": "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal RodoviÃ¡rio), via Alcochete e Samouco</span>",
    "volta": "<span> Montijo (Terminal RodoviÃ¡rio), via Alcochete e Samouco</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
    "circular": "",
    "ida_short": "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
    "volta_short": "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
    "circular_short": ""
  },
  "4704": {
    "ida": "<span>Atalaia </span><span>ğŸ –</span><span class='ultimo'> Lisboa (Oriente)</span>",
    "volta": "<span> Lisboa (Oriente)</span><span>ğŸ –</span><span class='ultimo'>Atalaia </span>",
    "circular": "",
    "ida_short": "<span>Atalaia </span><span>ğŸ –</span><span class='ultimo'> Lisboa </span>",
    "volta_short": "<span> Lisboa </span><span>ğŸ –</span><span class='ultimo'>Atalaia </span>",
    "circular_short": ""
  },
  "4702": {
    "ida": "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> Valbom</span>",
    "volta": "<span> Valbom</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
    "circular": "",
    "ida_short": "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> Valbom</span>",
    "volta_short": "<span> Valbom</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
    "circular_short": ""
  },
  "4705": {
    "ida": "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> Samouco</span>",
    "volta": "<span> Samouco</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
    "circular": "",
    "ida_short": "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> Samouco</span>",
    "volta_short": "<span> Samouco</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
    "circular_short": ""
  },
  "4707": {
    "ida": "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal RodoviÃ¡rio)</span>",
    "volta": "<span> Montijo (Terminal RodoviÃ¡rio)</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
    "circular": "",
    "ida_short": "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
    "volta_short": "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
    "circular_short": ""
  },
  "4725": {
    "ida": "<span>Lisboa (Sete Rios) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Sete Rios) </span>",
    "circular": "",
    "ida_short": "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
    "circular_short": ""
  },
  "4720": {
    "ida": "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
    "circular": "",
    "ida_short": "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
    "circular_short": ""
  },
  "4715": {
    "ida": "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS), via Pinhal Novo</span>",
    "volta": "<span> SetÃºbal (ITS), via Pinhal Novo</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
    "circular": "",
    "ida_short": "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
    "circular_short": ""
  },
  "4710": {
    "ida": "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
    "volta": "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
    "circular": "",
    "ida_short": "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
    "volta_short": "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
    "circular_short": ""
  },
  "4711": {
    "ida": "",
    "volta": "<span> Pinhal Novo</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
    "circular": "",
    "ida_short": "",
    "volta_short": "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
    "circular_short": ""
  },
  "4706": {
    "ida": "<span>SÃ£o Francisco </span><span>ğŸ –</span><span class='ultimo'> Lisboa (Oriente)</span>",
    "volta": "<span> Lisboa (Oriente)</span><span>ğŸ –</span><span class='ultimo'>SÃ£o Francisco </span>",
    "circular": "",
    "ida_short": "<span>SÃ£o Fran</span><span>ğŸ –</span><span class='ultimo'> Lisboa </span>",
    "volta_short": "<span> Lisboa </span><span>ğŸ –</span><span class='ultimo'>SÃ£o Fran</span>",
    "circular_short": ""
  },
  "4905": {
    "ida": "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> Vendas Novas</span>",
    "volta": "<span> Vendas Novas</span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
    "circular": "",
    "ida_short": "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> Vendas </span>",
    "volta_short": "<span> Vendas </span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
    "circular_short": ""
  },
  "4901": {
    "ida": "<span>Landeira </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
    "volta": "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Landeira </span>",
    "circular": "",
    "ida_short": "<span>Landeira</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
    "volta_short": "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Landeira</span>",
    "circular_short": ""
  },
  "4902": {
    "ida": "<span>Landeira </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
    "volta": "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Landeira </span>",
    "circular": "",
    "ida_short": "<span>Landeira</span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
    "volta_short": "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Landeira</span>",
    "circular_short": ""
  },
  "4906": {
    "ida": "<span>SetÃºbal (ITS) </span><span>ğŸ –</span><span class='ultimo'> Vendas Novas, via Landeira</span>",
    "volta": "<span> Vendas Novas, via Landeira</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (ITS) </span>",
    "circular": "",
    "ida_short": "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Vendas </span>",
    "volta_short": "<span> Vendas </span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
    "circular_short": ""
  }
};
const service_id = {
  "1": "DiÃ¡rio",
  "2": "Dias Ãºteis e domingos/feriados todo o ano",
  "3": "Dias Ãºteis e sÃ¡bados todo o ano",
  "4": "Dias Ãºteis todo o ano",
  "5": "Domingos/feriados todo o ano",
  "7": "SÃ¡bados (exceto feriados) todo o ano",
  "8": "SÃ¡bados, domingos/feriados todo o ano",
  "11": "Sextas-feiras e sÃ¡bados (exceto feriados) todo o ano",
  "15": "Dias Ãºteis de perÃ­odo escolar",
  "20": "Quartas, quintas e sextas-feiras (exceto feriados) de perÃ­odo escolar",
  "21": "Quartas-feiras (exceto feriados) de perÃ­odo escolar",
  "25": "Segundas e terÃ§as-feiras (exceto feriados) de perÃ­odo escolar",
  "28": "Segundas, terÃ§as, quintas e sextas-feiras (exceto feriados) de perÃ­odo escolar",
  "30": "Sextas-feiras (exceto feriados) de perÃ­odo escolar",
  "36": "Dias Ãºteis exceto verÃ£o",
  "38": "SÃ¡bados (exceto feriados) exceto verÃ£o",
  "39": "SÃ¡bados, domingos/feriados exceto verÃ£o",
  "40": "Todos os dias exceto verÃ£o",
  "41": "Dias Ãºteis de fÃ©rias escolares e verÃ£o",
  "50": "Dias Ãºteis de fÃ©rias escolares exceto verÃ£o",
  "51": "Dias Ãºteis de verÃ£o",
  "52": "SÃ¡bados de verÃ£o",
  "53": "Domingos/feriados de verÃ£o",
  "54": "SÃ¡bados, domingos/feriados de verÃ£o",
  "56": "Todos os dias de verÃ£o",
  "60": "Dias Ãºteis de fÃ©rias escolares e verÃ£o e sÃ¡bados (exceto feriados) todo o ano",
  "62": "Dias Ãºteis de perÃ­odo escolar e sÃ¡bados (exceto feriados) de fÃ©rias escolares e verÃ£o",
  "63": "Dias Ãºteis de perÃ­odo escolar e sÃ¡bados (exceto feriados) todo o ano",
  "66": "Dias Ãºteis de perÃ­odo escolar e segundas-feiras (exceto feriados) de verÃ£o",
  "69": "Segundo domingo de cada mÃªs",
  "77": "Dias Ãºteis todo o ano e sÃ¡bados (exceto feriados) de fÃ©rias escolares e verÃ£o",
  "87": "Domingos exceto primeiro de cada mÃªs todo o ano",
  "97": "Primeiro domingo de cada mÃªs todo o ano",
  "100": "SÃ¡bados (exceto feriados) todo o ano e dias Ãºteis de perÃ­odo escolar",
  "102": "SÃ¡bados (exceto feriados) todo o ano e dias Ãºteis de verÃ£o",
  "109": "Domingos/feriados todo o ano e dias Ãºteis de fÃ©rias escolares e verÃ£o",
  "111": "SÃ¡bados, domingos/feriados todo o ano e dias Ãºteis de verÃ£o",
  "112": "SÃ¡bados, domingos/feriados todo o ano e dias Ãºteis exceto verÃ£o",
  "113": "Todos os dias de fÃ©rias escolares e verÃ£o domingos/feriados de perÃ­odo escolar",
  "115": "Todos os dias de perÃ­odo escolar e sÃ¡bados, domingos/feriados de fÃ©rias escolares e verÃ£o",
  "118": "Todos os dias entre 10 de junho e 15 de setembro",
  "119": "SÃ¡bados, domingos/feriados entre 10 de junho e 15 de setembro e dias Ãºteis entre 4 de julho e 15 de setembro",
  "120": "SÃ¡bados, domingos/feriados entre 10 de junho e 15 de setembro e dias Ãºteis de agosto",
  "121": "SÃ¡bados, domingos/feriados entre 10 de junho e 15 de setembro e dias Ãºteis entre 1 de julho e 15 de setembro"
};
const linhas_e_seus_horarios_nomes_ficheiros = {
  "3001": {
    "Almada (Cristo Rei) - Cacilhas (Terminal)": "3001_0_1.json",
    "Cacilhas (Terminal) - Almada (Cristo Rei)": "3001_0_2.json"
  },
  "3003": {
    "Almada Forum - Cacilhas (Terminal)": "3003_0_1.json",
    "Cacilhas (Terminal) - Almada Forum": "3003_0_2.json"
  },
  "3004": {
    "Almada Forum - Marisol": "3004_0_1.json",
    "Marisol - Almada Forum": "3004_0_2.json"
  },
  "3007": {
    "Bairro Fundo Fomento - Cacilhas (Terminal)": "3007_0_1.json",
    "Cacilhas (Terminal) - Bairro Fundo Fomento": "3007_0_2.json"
  },
  "3008": {
    "BanÃ¡tica - Quintinha": "3008_0_1.json",
    "Quintinha - BanÃ¡tica": "3008_0_2.json"
  },
  "3009": {
    "Cacilhas (terminal - Trafaria (Terminal)": "3009_0_1.json",
    "Trafaria (Terminal) - Cacilhas (terminal": "3009_0_2.json"
  },
  "3010": {
    "Cacilhas (Terminal) - Charneca da Caparica": "3010_0_1.json",
    "Charneca da Caparica - Cacilhas (Terminal)": "3010_0_2.json"
  },
  "3011": {
    "Cacilhas (Terminal) - Costa da Caparica": "3011_0_1.json",
    "Cacilhas (Terminal) - Costa da Caparica, via Escola JosÃ© Cardoso Pires": "3011_1_1.json",
    "Costa da Caparica - Cacilhas (Terminal)": "3011_0_2.json",
    "Costa da Caparica, via Escola JosÃ© Cardoso Pires - Cacilhas (Terminal)": "3011_1_2.json"
  },
  "3012": {
    "Cacilhas (Terminal) - Fonte da Telha": "3012_0_1.json",
    "Fonte da Telha - Cacilhas (Terminal)": "3012_0_2.json"
  },
  "3013": {
    "Cacilhas (Terminal) - Monte da Caparica": "3013_0_1.json",
    "Monte da Caparica - Cacilhas (Terminal)": "3013_0_2.json"
  },
  "3014": {
    "Cacilhas (terminal) - Raposeira": "3014_0_1.json",
    "Raposeira - Cacilhas (terminal)": "3014_0_2.json"
  },
  "3015": {
    "Charneca da Caparica - Cova do Vapor": "3015_0_1.json",
    "Cova do Vapor - Charneca da Caparica": "3015_0_2.json"
  },
  "3017": {
    "Charneca da Caparica - Pragal (EstaÃ§Ã£o)": "3017_0_1.json",
    "Pragal (EstaÃ§Ã£o) - Charneca da Caparica": "3017_0_2.json"
  },
  "3019": {
    "Charneca da Caparica - Trafaria (Terminal)": "3019_0_1.json",
    "Trafaria (Terminal) - Charneca da Caparica": "3019_0_2.json"
  },
  "3021": {
    "Costa da Caparica - Monte da Caparica (FCT)": "3021_0_1.json",
    "Monte da Caparica (FCT) - Costa da Caparica": "3021_0_2.json"
  },
  "3022": {
    "Costa da Caparica (Terminal) - Hospital Garcia de Orta": "3022_0_1.json",
    "Hospital Garcia de Orta - Costa da Caparica (Terminal)": "3022_0_2.json"
  },
  "3024": {
    "Costa da Caparica (Terminal) - Pragal (EstaÃ§Ã£o)": "3024_0_1.json",
    "Pragal (EstaÃ§Ã£o) - Costa da Caparica (Terminal)": "3024_0_2.json"
  },
  "3026": {
    "Cova da Piedade - Hospital Garcia de Orta": "3026_0_1.json",
    "Cova da Piedade - Hospital Garcia de Orta, via Base Naval": "3026_1_1.json",
    "Hospital Garcia de Orta - Cova da Piedade": "3026_0_2.json",
    "Hospital Garcia de Orta, via Base Naval - Cova da Piedade": "3026_1_2.json"
  },
  "3027": {
    "Hospital Garcia de Orta - Sobreda": "3027_0_1.json",
    "Pragal (EstaÃ§Ã£o) - Sobreda, via Hospital Garcia de Orta": "3027_1_1.json",
    "Sobreda - Hospital Garcia de Orta": "3027_0_2.json"
  },
  "3030": {
    "Fonte da Telha - Monte da Caparica (FCT)": "3030_0_1.json",
    "Monte da Caparica (FCT) - Fonte da Telha": "3030_0_2.json"
  },
  "3034": {
    "Porto BrandÃ£o (Terminal) - Quinta do Texugo": "3034_0_1.json",
    "Quinta do Texugo - Porto BrandÃ£o (Terminal)": "3034_0_2.json"
  },
  "3035": {
    "Pragal (EstaÃ§Ã£o) - Quinta do Texugo": "3035_0_1.json",
    "Quinta do Texugo - Pragal (EstaÃ§Ã£o)": "3035_0_2.json"
  },
  "3036": {
    "Pragal (EstaÃ§Ã£o) - Vale Flores": "3036_0_1.json",
    "Vale Flores - Pragal (EstaÃ§Ã£o)": "3036_0_2.json"
  },
  "3101": {
    "Amora - Foros de Amora (EstaÃ§Ã£o)": "3101_0_1.json",
    "Foros de Amora (EstaÃ§Ã£o) - Amora": "3101_0_2.json"
  },
  "3103": {
    "Corroios (EstaÃ§Ã£o) - Paio Pires (Farinheiras)": "3103_0_1.json",
    "Paio Pires (Farinheiras) - Corroios (EstaÃ§Ã£o)": "3103_0_2.json"
  },
  "3104": {
    "Corroios (EstaÃ§Ã£o) - Vale de MilhaÃ§os": "3104_0_1.json",
    "Vale de MilhaÃ§os - Corroios (EstaÃ§Ã£o)": "3104_0_2.json"
  },
  "3105": {
    "FernÃ£o Ferro - Fogueteiro (EstaÃ§Ã£o)": "3105_0_1.json",
    "Fogueteiro (EstaÃ§Ã£o) - FernÃ£o Ferro": "3105_0_2.json"
  },
  "3107": {
    "Laranjeiras - Marco do Grilo": "3107_0_1.json",
    "Marco do Grilo - Laranjeiras": "3107_0_2.json"
  },
  "3108": {
    "Fogueteiro - Foros de Amora (EstaÃ§Ã£o)": "3108_0_1.json",
    "Foros de Amora (EstaÃ§Ã£o) - Fogueteiro": "3108_0_2.json"
  },
  "3109": {
    "Fogueteiro (EstaÃ§Ã£o) - Parque Empresarial do Seixal": "3109_0_1.json",
    "Parque Empresarial do Seixal - Fogueteiro (EstaÃ§Ã£o)": "3109_0_2.json"
  },
  "3110": {
    "Fogueteiro (EstaÃ§Ã£o) - Redondos": "3110_0_1.json",
    "Redondos - Fogueteiro (EstaÃ§Ã£o)": "3110_0_2.json"
  },
  "3111": {
    "Fogueteiro (EstaÃ§Ã£o) - Seixal (Terminal Fluvial)": "3111_0_1.json",
    "Seixal (Terminal Fluvial) - Fogueteiro (EstaÃ§Ã£o)": "3111_0_2.json"
  },
  "3112": {
    "Fogueteiro (EstaÃ§Ã£o) - Seixal (Terminal Fluvial), via Paio Pires": "3112_0_1.json",
    "Seixal (Terminal Fluvial), via Paio Pires - Fogueteiro (EstaÃ§Ã£o)": "3112_0_2.json"
  },
  "3113": {
    "Fogueteiro (EstaÃ§Ã£o) - Seixal (Terminal Fluvial), via Quinta do Cabral": "3113_0_1.json",
    "Seixal (Terminal Fluvial), via Quinta do Cabral - Fogueteiro (EstaÃ§Ã£o)": "3113_0_2.json"
  },
  "3114": {
    "Foros de Amora - Paio Pires (Quinta FlamÃ¢ncia)": "3114_0_1.json",
    "Paio Pires (Quinta FlamÃ¢ncia) - Foros de Amora": "3114_0_2.json"
  },
  "3119": {
    "Pinhal Conde Cunha - Seixal (Terminal Fluvial)": "3119_0_1.json",
    "Seixal (Terminal Fluvial) - Pinhal Conde Cunha": "3119_0_2.json"
  },
  "3120": {
    "Redondos - Seixal (Terminal Fluvial)": "3120_0_1.json",
    "Seixal (Terminal Fluvial) - Redondos": "3120_0_2.json"
  },
  "3121": {
    "Seixal | Circular": "3121_0_3.json"
  },
  "3201": {
    "Aldeia do Meco - Sesimbra (Terminal), via Aiana": "3201_0_1.json",
    "Sesimbra (Terminal), via Aiana - Aldeia do Meco": "3201_0_2.json"
  },
  "3202": {
    "Alfarim - Sesimbra (Terminal), via Aiana": "3202_0_1.json",
    "Sesimbra (Terminal), via Aiana - Alfarim": "3202_0_2.json"
  },
  "3203": {
    "Azoia - Sesimbra (Terminal)": "3203_0_1.json",
    "Sesimbra (Terminal) - Azoia": "3203_0_2.json"
  },
  "3204": {
    "Azoia - Sesimbra (Terminal), via Serra da Azoia": "3204_0_1.json",
    "Sesimbra (Terminal), via Serra da Azoia - Azoia": "3204_0_2.json"
  },
  "3205": {
    "Cabo Espichel - Sesimbra (Terminal)": "3205_0_1.json",
    "Sesimbra (Terminal) - Cabo Espichel": "3205_0_2.json"
  },
  "3206": {
    "Carrasqueira - Sesimbra (Terminal)": "3206_0_1.json",
    "Sesimbra (Terminal) - Carrasqueira": "3206_0_2.json"
  },
  "3207": {
    "Carrasqueira - Sesimbra (Terminal), via Valbom e Sampaio": "3207_0_1.json",
    "Sesimbra (Terminal), via Valbom e Sampaio - Carrasqueira": "3207_0_2.json"
  },
  "3208": {
    "Fetais - Sesimbra (Terminal)": "3208_0_1.json",
    "Sesimbra (Terminal) - Fetais": "3208_0_2.json"
  },
  "3209": {
    "Fornos - Sesimbra (Terminal), via Aiana": "3209_0_1.json",
    "Sesimbra (Terminal), via Aiana - Fornos": "3209_0_2.json"
  },
  "3210": {
    "Lagoa de Albufeira - Sesimbra (Terminal)": "3210_0_1.json",
    "Lagoa de Albufeira - Sesimbra (Terminal), via Praia da Lagoa de Albufeira e do Meco": "3210_1_1.json",
    "Sesimbra (Terminal) - Lagoa de Albufeira": "3210_0_2.json",
    "Sesimbra (Terminal), via Praia da Lagoa de Albufeira e do Meco - Lagoa de Albufeira": "3210_1_2.json"
  },
  "3212": {
    "MaÃ§Ã£ (Rua Macieira) - Sesimbra (Terminal)": "3212_0_1.json",
    "Sesimbra (Terminal) - MaÃ§Ã£ (Rua Macieira)": "3212_0_2.json"
  },
  "3213": {
    "Pinhal de Cima - Sesimbra (Terminal) | Circular": "3213_0_3.json"
  },
  "3218": {
    "Sesimbra (Parque de Campismo) - Sesimbra (Terminal)": "3218_1_1.json",
    "Sesimbra (Porto de Abrigo) - Sesimbra (Terminal)": "3218_0_1.json",
    "Sesimbra (Terminal) - Sesimbra (Parque de Campismo)": "3218_1_2.json",
    "Sesimbra (Terminal) - Sesimbra (Porto de Abrigo)": "3218_0_2.json"
  },
  "3220": {
    "Sesimbra | Circular": "3220_0_1.json",
    "Sesimbra | Circular (2)": "3220_0_2.json"
  },
  "3221": {
    "Sesimbra (Terminal), via Sampaio - Valbom": "3221_0_2.json",
    "Valbom - Sesimbra (Terminal), via Sampaio": "3221_0_1.json"
  },
  "3222": {
    "Quinta do Conde | Circular": "3222_0_1.json",
    "Quinta do Conde | Circular (2)": "3222_0_2.json"
  },
  "3502": {
    "Almada Forum - Paio Pires (Centro)": "3502_0_1.json",
    "Paio Pires (Centro) - Almada Forum": "3502_0_2.json"
  },
  "3506": {
    "Cacilhas (Terminal) - Corroios (EstaÃ§Ã£o), via Miratejo": "3506_0_1.json",
    "Corroios (EstaÃ§Ã£o), via Miratejo - Cacilhas (Terminal)": "3506_0_2.json"
  },
  "3507": {
    "Cacilhas (Terminal) - Marisol": "3507_0_1.json",
    "Marisol - Cacilhas (Terminal)": "3507_0_2.json"
  },
  "3508": {
    "Cacilhas (Terminal) - Paio Pires (Centro)": "3508_0_1.json",
    "Foros de Amora (EstaÃ§Ã£o) - Paio Pires (Centro)": "3508_1_1.json",
    "Paio Pires (Centro) - Cacilhas (Terminal)": "3508_0_2.json",
    "Paio Pires (Centro) - Foros de Amora (EstaÃ§Ã£o)": "3508_2_2.json"
  },
  "3509": {
    "Cacilhas (Terminal) - Paio Pires (Centro), via Amora e Corroios (EstaÃ§Ã£o)": "3509_1_1.json",
    "Cacilhas (Terminal) - Paio Pires (Centro), via Seixal (Terminal Fluvial) e Amora": "3509_0_1.json",
    "Paio Pires (Centro), via Amora e Corroios (EstaÃ§Ã£o) - Cacilhas (Terminal)": "3509_1_2.json",
    "Paio Pires (Centro), via Seixal (Terminal Fluvial) e Amora - Cacilhas (Terminal)": "3509_0_2.json"
  },
  "3510": {
    "Cacilhas (Terminal) - Pilotos": "3510_0_1.json",
    "Pilotos - Cacilhas (Terminal)": "3510_0_2.json"
  },
  "3511": {
    "Cacilhas (Terminal) - Pinheirinho": "3511_0_1.json",
    "Cacilhas (Terminal) - Vale de MilhaÃ§os": "3511_1_1.json",
    "Pinheirinho - Cacilhas (Terminal)": "3511_0_2.json"
  },
  "3512": {
    "Cacilhas (Terminal) - Quinta Princesa": "3512_0_1.json",
    "Quinta Princesa - Cacilhas (Terminal)": "3512_0_2.json"
  },
  "3513": {
    "Cacilhas (Terminal) - Santa Marta do Pinhal": "3513_0_1.json",
    "Cacilhas (Terminal) - Santa Marta do Pinhal (CemitÃ©rio)": "3513_1_1.json",
    "Santa Marta do Pinhal (CemitÃ©rio) - Cacilhas (Terminal)": "3513_1_2.json",
    "Santa Marta do Pinhal - Cacilhas (Terminal)": "3513_0_2.json"
  },
  "3514": {
    "Cacilhas (Terminal) - Vale de MilhaÃ§os": "3514_0_1.json",
    "Vale de MilhaÃ§os - Cacilhas (Terminal)": "3514_0_2.json"
  },
  "3515": {
    " Corroios - Caparica (Pilotos)": "3515_0_2.json",
    "Caparica (Pilotos) -  Corroios": "3515_0_1.json"
  },
  "3516": {
    "Charneca da Caparica - Corroios (EstaÃ§Ã£o)": "3516_0_1.json",
    "Corroios (EstaÃ§Ã£o) - Charneca da Caparica": "3516_0_2.json"
  },
  "3517": {
    "Chegadinho - Corroios (EstaÃ§Ã£o)": "3517_0_1.json",
    "Corroios (EstaÃ§Ã£o) - Chegadinho": "3517_0_2.json"
  },
  "3518": {
    "Corroios (EstaÃ§Ã£o) - Vale de Figueira": "3518_0_1.json",
    "Vale de Figueira - Corroios (EstaÃ§Ã£o)": "3518_0_2.json"
  },
  "3519": {
    "Corroios (EstaÃ§Ã£o) - Costa da Caparica (Terminal)": "3519_0_2.json",
    "Costa da Caparica (Terminal) - Corroios (EstaÃ§Ã£o)": "3519_0_1.json"
  },
  "3520": {
    "Costa da Caparica (Terminal) - Quinta do Brasileiro": "3520_0_1.json",
    "Quinta do Brasileiro - Costa da Caparica (Terminal)": "3520_0_2.json"
  },
  "3521": {
    "Cruz de Pau - Fonta da Telha": "3521_0_1.json",
    "Fonta da Telha - Cruz de Pau": "3521_0_2.json"
  },
  "3522": {
    "Fonte da Telha - Paio Pires (Centro)": "3522_0_1.json",
    "Paio Pires (Centro) - Fonte da Telha": "3522_0_2.json"
  },
  "3523": {
    "Fonte da Telha - Paio Pires (Quinta FlamÃ¢ncia), via Seixal (Terminal Fluvial) e Foros de Amora (EstaÃ§Ã£o)": "3523_0_1.json",
    "Paio Pires (Quinta FlamÃ¢ncia), via Seixal (Terminal Fluvial) e Foros de Amora (EstaÃ§Ã£o) - Fonte da Telha": "3523_0_2.json"
  },
  "3524": {
    "Hospital Garcia de Orta - Marisol": "3524_0_1.json",
    "Marisol - Hospital Garcia de Orta": "3524_0_2.json"
  },
  "3525": {
    "Hospital Garcia de Orta - Miratejo": "3525_0_1.json",
    "Miratejo - Hospital Garcia de Orta": "3525_0_2.json"
  },
  "3526": {
    "Laranjeiro - Pinheirinho": "3526_0_1.json",
    "Pinheirinho - Laranjeiro": "3526_0_2.json"
  },
  "3527": {
    "Monte da Caparica (FCT) - Almada Forum": "3527_1_1.json",
    "Monte da Caparica (FCT) - Paio Pires (Bairro Cucena)": "3527_0_1.json",
    "Paio Pires (Bairro Cucena) - Monte da Caparica (FCT)": "3527_0_2.json"
  },
  "3528": {
    "Monte da Caparica (FCT) - Paio Pires (Centro)": "3528_0_1.json",
    "Paio Pires (Centro) - Monte da Caparica (FCT)": "3528_0_2.json"
  },
  "3535": {
    "Cacilhas (Terminal) - Quinta do Conde": "3535_0_1.json",
    "Quinta do Conde - Cacilhas (Terminal)": "3535_0_2.json"
  },
  "3536": {
    "Cacilhas (Terminal) - Sesimbra (Terminal)": "3536_0_1.json",
    "Sesimbra (Terminal) - Cacilhas (Terminal)": "3536_0_2.json"
  },
  "3543": {
    "Quinta do Conde, via Estrada de Coina - Coina (EstaÃ§Ã£o)": "3543_0_2.json"
  },
  "3545": {
    "Corroios (EstaÃ§Ã£o) - Sesimbra (Terminal)": "3545_0_1.json",
    "Sesimbra (Terminal) - Corroios (EstaÃ§Ã£o)": "3545_0_2.json"
  },
  "3546": {
    "Cruz de Pau - Quinta do Conde": "3546_0_1.json",
    "Quinta do Conde - Cruz de Pau": "3546_0_2.json"
  },
  "3547": {
    "Fogueteiro (EstaÃ§Ã£o) - Quinta do Conde": "3547_0_1.json",
    "Quinta do Conde - Fogueteiro (EstaÃ§Ã£o)": "3547_0_2.json"
  },
  "3549": {
    "Quinta do Conde - Sesimbra (Terminal), via Marco do Grilo": "3549_1_1.json",
    "Sesimbra (Terminal), via Marco do Grilo - Quinta do Conde": "3549_1_2.json"
  },
  "3601": {
    "Barreiro - Cova da Piedade (Metro)": "3601_0_1.json",
    "Cova da Piedade (Metro) - Barreiro": "3601_0_2.json"
  },
  "3605": {
    "Cacilhas (Terminal) - SetÃºbal (ITS), via AzeitÃ£o": "3605_0_1.json",
    "SetÃºbal (ITS), via AzeitÃ£o - Cacilhas (Terminal)": "3605_0_2.json"
  },
  "3615": {
    "Barreiro - Seixal": "3615_0_1.json",
    "Seixal - Barreiro": "3615_0_2.json"
  },
  "3620": {
    "Coina (EstaÃ§Ã£o) - Quinta do Conde": "3620_0_1.json",
    "Quinta do Conde - Coina (EstaÃ§Ã£o)": "3620_0_2.json"
  },
  "3626": {
    "Coina (EstaÃ§Ã£o) - Vila Fresca de AzeitÃ£o": "3626_1_1.json",
    "Vila Fresca de AzeitÃ£o - Coina (EstaÃ§Ã£o)": "3626_0_2.json",
    "Vila Fresca de AzeitÃ£o - Coina (EstaÃ§Ã£o) (2)": "3626_1_2.json"
  },
  "3635": {
    "Coina (EstaÃ§Ã£o) - Sesimbra (Terminal), via AzeitÃ£o": "3635_1_1.json",
    "Coina (EstaÃ§Ã£o) - Sesimbra (Terminal), via AzeitÃ£o e Sampaio": "3635_0_1.json",
    "Sesimbra (Terminal), via AzeitÃ£o - Coina (EstaÃ§Ã£o)": "3635_1_2.json",
    "Sesimbra (Terminal), via AzeitÃ£o e Sampaio - Coina (EstaÃ§Ã£o)": "3635_0_2.json"
  },
  "3640": {
    "Azoia - Vila Nogueira de AzeitÃ£o": "3640_0_1.json",
    "Vila Nogueira de AzeitÃ£o - Azoia": "3640_0_2.json"
  },
  "3702": {
    "Almada (Parque Urbano) - Lisboa (C. UniversitÃ¡ria)": "3702_0_1.json",
    "Lisboa (C. UniversitÃ¡ria) - Almada (Parque Urbano)": "3702_0_2.json"
  },
  "3703": {
    "Almada (Parque Urbano) - Lisboa (AlcÃ¢ntara)": "3703_1_1.json",
    "Almada (Parque Urbano) - Lisboa (Sete Rios)": "3703_0_1.json",
    "Lisboa (Sete Rios) - Almada (Parque Urbano)": "3703_0_2.json"
  },
  "3709": {
    "Costa da Caparica (Terminal) - Lisboa (M. Pombal)": "3709_0_1.json",
    "Lisboa (M. Pombal) - Costa da Caparica (Terminal)": "3709_0_2.json"
  },
  "3710": {
    "Costa da Caparica (Terminal) - Lisboa (Sete Rios)": "3710_0_1.json",
    "Lisboa (Sete Rios) - Costa da Caparica (Terminal)": "3710_0_2.json"
  },
  "3711": {
    "Lisboa (Sete Rios) - Monte da Caparica (FCT)": "3711_0_2.json",
    "Monte da Caparica (FCT) - Lisboa (Sete Rios)": "3711_0_1.json"
  },
  "3715": {
    "Lisboa (M. Pombal) - Santa Marta do Pinhal": "3715_0_1.json",
    "Santa Marta do Pinhal - Lisboa (M. Pombal)": "3715_0_2.json"
  },
  "3716": {
    "Lisboa (Sete Rios) - Marisol": "3716_0_1.json",
    "Marisol - Lisboa (Sete Rios)": "3716_0_2.json"
  },
  "3717": {
    "Lisboa (Sete Rios) - Quinta do Brasileiro": "3717_0_1.json",
    "Quinta do Brasileiro - Lisboa (Sete Rios)": "3717_0_2.json"
  },
  "3720": {
    "Lisboa (Sete Rios) - Quinta do Conde": "3720_0_1.json",
    "Quinta do Conde - Lisboa (Sete Rios)": "3720_0_2.json"
  },
  "3721": {
    "Lisboa (Sete Rios) - Sesimbra (Terminal)": "3721_0_1.json",
    "Sesimbra (Terminal) - Lisboa (Sete Rios)": "3721_0_2.json"
  },
  "4001": {
    "Alcochete | Circular": "4001_0_3.json"
  },
  "4002": {
    "SÃ£o Francisco | Circular": "4002_0_3.json"
  },
  "4101": {
    "Alhos Vedros (Escola JosÃ© Afonso) - Arroteias": "4101_0_1.json",
    "Arroteias - Alhos Vedros (Escola JosÃ© Afonso)": "4101_0_2.json"
  },
  "4102": {
    "CabeÃ§o Verde - Sarilhos Pequenos": "4102_0_1.json",
    "Sarilhos Pequenos - CabeÃ§o Verde": "4102_0_2.json"
  },
  "4103": {
    "Moita (EstaÃ§Ã£o) - Sarilhos Pequenos": "4103_0_1.json",
    "Sarilhos Pequenos - Moita (EstaÃ§Ã£o)": "4103_0_2.json"
  },
  "4104": {
    "Moita | Circular": "4104_0_3.json"
  },
  "4201": {
    "Afonsoeiro - Bairro da Liberdade": "4201_0_1.json",
    "Bairro da Liberdade - Afonsoeiro": "4201_0_2.json"
  },
  "4202": {
    "Afonsoeiro - Bairro do Saldanha via Bairro da CalÃ§ada": "4202_0_1.json",
    "Bairro do Saldanha via Bairro da CalÃ§ada - Afonsoeiro": "4202_0_2.json"
  },
  "4203": {
    "Afonsoeiro - Montijo (Terminal Fluvial) via Bairro da Liberdade": "4203_0_1.json",
    "Montijo (Terminal Fluvial) via Bairro da Liberdade - Afonsoeiro": "4203_0_2.json",
    "Montijo (Terminal Fluvial) - Bairro Areias": "4203_1_2.json"
  },
  "4204": {
    "Bairro do CharqueirÃ£o - Montijo (Terminal Fluvial)": "4204_0_1.json",
    "Montijo (Terminal Fluvial) - Bairro do CharqueirÃ£o": "4204_0_2.json"
  },
  "4205": {
    "Bairro do CharqueirÃ£o - Montijo (Terminal Fluvial) via Vale Porim": "4205_0_1.json",
    "Montijo (Terminal Fluvial) via Vale Porim - Bairro do CharqueirÃ£o": "4205_0_2.json",
    "Montijo (Terminal RodoviÃ¡rio) - Montijo (Ã�rea Comercial)": "4205_1_2.json"
  },
  "4206": {
    "Bairro Esteval - Montijo (Terminal Fluvial)": "4206_0_1.json",
    "Montijo (Terminal Fluvial) - Bairro Esteval": "4206_0_2.json"
  },
  "4207": {
    "Montijo (Ã�rea Comercial) - Montijo (Terminal Fluvial)": "4207_0_1.json",
    "Montijo (Escola Joaquim Serra) - Montijo (Terminal Fluvial)": "4207_1_1.json",
    "Montijo (Terminal Fluvial) - Montijo (Ã�rea Comercial)": "4207_0_2.json",
    "Montijo (Terminal Fluvial) - Bela Vista": "4207_2_2.json"
  },
  "4208": {
    "Montijo (Terminal RodoviÃ¡rio) - Sarilhos Grandes (Estr. 4 Marcos)": "4208_0_1.json",
    "Sarilhos Grandes (Estr. 4 Marcos) - Montijo (Terminal RodoviÃ¡rio)": "4208_0_2.json"
  },
  "4210": {
    "Canha - Foros Boavista": "4210_0_1.json",
    "Foros Boavista - Canha": "4210_0_2.json"
  },
  "4211": {
    "Craveiras - PegÃµes | Circular": "4211_0_3.json"
  },
  "4212": {
    "Foros Boavista - PegÃµes": "4212_0_1.json",
    "PegÃµes - Foros Boavista": "4212_0_2.json"
  },
  "4301": {
    "Palmela (Centro) - Palmela (Terminal)": "4301_0_1.json",
    "Palmela (Terminal) - Palmela (Centro)": "4301_0_2.json"
  },
  "4302": {
    "Palmela (EstaÃ§Ã£o) - Palmela (Terminal)": "4302_0_1.json",
    "Palmela (Terminal) - Palmela (EstaÃ§Ã£o)": "4302_0_2.json"
  },
  "4303": {
    "Palmela | Circular": "4303_0_3.json"
  },
  "4304": {
    "Palmela (Terminal) - Penalva": "4304_0_1.json",
    "Penalva - Palmela (Terminal)": "4304_0_2.json"
  },
  "4305": {
    "Brejos do Assa - Palmela (Terminal)": "4305_0_1.json",
    "Palmela (Terminal) - Brejos do Assa": "4305_0_2.json"
  },
  "4306": {
    "Cabanas - Palmela (Terminal)": "4306_0_1.json",
    "Palmela (Terminal) - Cabanas": "4306_0_2.json"
  },
  "4307": {
    "Loja Nova - Palmela (Terminal)": "4307_0_1.json",
    "Palmela (Terminal) - Loja Nova": "4307_0_2.json"
  },
  "4308": {
    "Palmela (Terminal) - Pinhal Novo (EstaÃ§Ã£o)": "4308_0_1.json",
    "Pinhal Novo (EstaÃ§Ã£o) - Palmela (Terminal)": "4308_0_2.json"
  },
  "4310": {
    "Ã�guas de Moura - PoceirÃ£o": "4310_0_1.json",
    "PoceirÃ£o - Ã�guas de Moura": "4310_0_2.json"
  },
  "4311": {
    "Asseiceira - PoceirÃ£o": "4311_0_1.json",
    "PoceirÃ£o - Asseiceira": "4311_0_2.json"
  },
  "4312": {
    "PoceirÃ£o - Vale Abrunheira (X) via Fernando PÃ³": "4312_0_1.json",
    "PoceirÃ£o - Vale Abrunheira (X)": "4312_1_1.json",
    "Vale Abrunheira (X) via Fernando PÃ³ - PoceirÃ£o": "4312_0_2.json",
    "Vale Abrunheira (X) - PoceirÃ£o": "4312_1_2.json",
    "Agualva de Cima via Fernando PÃ³ - PoceirÃ£o": "4312_2_2.json"
  },
  "4313": {
    "Cabanas - Penalva": "4313_0_1.json",
    "Penalva - Cabanas": "4313_0_2.json"
  },
  "4320": {
    "Pinhal Novo | Circular": "4320_0_3.json"
  },
  "4321": {
    "Pinhal Novo - Qta do Anjo": "4321_0_1.json",
    "Qta do Anjo - Pinhal Novo": "4321_0_2.json"
  },
  "4322": {
    "Pinhal Novo - Rio Frio": "4322_0_1.json",
    "Pinhal Novo - Rio Frio via ColÃ©gio A Palmeira": "4322_1_1.json",
    "Rio Frio - Pinhal Novo": "4322_0_2.json",
    "Rio Frio via ColÃ©gio A Palmeira - Pinhal Novo": "4322_1_2.json"
  },
  "4401": {
    "Cachofarra - SetÃºbal (Hospital)": "4401_0_1.json",
    "SetÃºbal (Hospital) - Cachofarra": "4401_0_2.json"
  },
  "4402": {
    "Estefanilha - SetÃºbal (ITS)": "4402_0_1.json",
    "SetÃºbal (ITS) - Estefanilha": "4402_0_2.json"
  },
  "4403": {
    "Fonte da Talha - SetÃºbal (Av. LuÃ­sa Todi)": "4403_0_1.json",
    "SetÃºbal (Av. LuÃ­sa Todi) - Fonte da Talha": "4403_0_2.json"
  },
  "4404": {
    "Interfaces SetÃºbal | Circular": "4404_0_3.json"
  },
  "4405": {},
  "4406": {
    "Manteigadas - SetÃºbal (Mercado)": "4406_0_1.json",
    "SetÃºbal (PolitÃ©cnico) - SetÃºbal (Mercado)": "4406_1_1.json",
    "SetÃºbal (Mercado) - Manteigadas": "4406_0_2.json",
    "SetÃºbal (Mercado) - SetÃºbal (PolitÃ©cnico)": "4406_1_2.json"
  },
  "4407": {
    "Manteigadas - SetÃºbal (Mercado) via Bairro da Carmona": "4407_0_1.json",
    "SetÃºbal (Av. Pinheiro) - SetÃºbal (Mercado) via Bairro da Carmona": "4407_1_1.json"
  },
  "4408": {
    "SetÃºbal (Mercado) via Bela Vista - Manteigadas": "4408_0_2.json"
  },
  "4409": {
    "Manteigadas - Viso": "4409_0_1.json",
    "Viso - Manteigadas": "4409_0_2.json"
  },
  "4410": {
    "Manteigadas (Esc. Profissional) - SetÃºbal (Alegro)": "4410_0_1.json",
    "SetÃºbal (Alegro) - Manteigadas (Esc. Profissional)": "4410_0_2.json"
  },
  "4411": {
    "Morgada - SetÃºbal (ITS)": "4411_0_1.json",
    "SetÃºbal (ITS) - Morgada": "4411_0_2.json"
  },
  "4412": {
    "Morgada - SetÃºbal (Mercado)": "4412_0_1.json",
    "SetÃºbal (Mercado) - Morgada": "4412_0_2.json"
  },
  "4413": {
    "SetÃºbal (Mercado) via Bela Vista - Morgada": "4413_0_2.json"
  },
  "4414": {
    "OutÃ£o (Hospital) - SetÃºbal (ITS)": "4414_0_1.json",
    "SetÃºbal (ITS) - OutÃ£o (Hospital)": "4414_0_2.json"
  },
  "4415": {
    "OutÃ£o (Hospital) - SetÃºbal (ITS) via vale da Rasca": "4415_0_1.json",
    "SetÃºbal (ITS) via vale da Rasca - OutÃ£o (Hospital)": "4415_0_2.json"
  },
  "4416": {
    "PoÃ§o Mouro - SetÃºbal (ITS)": "4416_0_1.json",
    "SetÃºbal (ITS) - PoÃ§o Mouro": "4416_0_2.json"
  },
  "4417": {
    "PoÃ§o Mouro - SetÃºbal (ITS) via Manteigadas": "4417_0_1.json",
    "SetÃºbal (ITS) via Manteigadas - PoÃ§o Mouro": "4417_0_2.json"
  },
  "4418": {
    "SetÃºbal (Alegro) - SetÃºbal (Av. 5 Outubro)": "4418_0_1.json",
    "SetÃºbal (Av. 5 Outubro) - SetÃºbal (Alegro)": "4418_0_2.json"
  },
  "4419": {
    "Brejos Canes - SetÃºbal (Saboaria)": "4419_0_1.json",
    "SetÃºbal (Saboaria) - Brejos Canes": "4419_0_2.json"
  },
  "4420": {
    "SetÃºbal (Alegro) - SetÃºbal (ITS)": "4420_0_1.json",
    "SetÃºbal (ITS) - SetÃºbal (Alegro)": "4420_0_2.json"
  },
  "4421": {
    "SetÃºbal (Bairro Camolas) - SetÃºbal (Casal Figueiras)": "4421_0_1.json",
    "SetÃºbal (Casal Figueiras) - SetÃºbal (Bairro Camolas)": "4421_0_2.json"
  },
  "4422": {
    "SetÃºbal (Bairro Camolas) - SetÃºbal (Casal Figueiras) via Bairro do Viso": "4422_0_1.json",
    "SetÃºbal (Bairro Camolas) - SetÃºbal (Bairro do Viso)": "4422_1_1.json",
    "SetÃºbal (Casal Figueiras) via Bairro do Viso - SetÃºbal (Bairro Camolas)": "4422_0_2.json",
    "SetÃºbal (Casal Figueiras) via Bairro do Viso - SetÃºbal (ITS)": "4422_2_2.json"
  },
  "4423": {
    "Amoreiras - SetÃºbal (Av. LuÃ­sa Todi)": "4423_0_1.json",
    "SetÃºbal (Av. LuÃ­sa Todi) - Amoreiras": "4423_0_2.json"
  },
  "4424": {
    "SetÃºbal (Bairro Viso) - Manteigadas": "4424_0_1.json",
    "Manteigadas - SetÃºbal (Bairro Viso)": "4424_0_2.json"
  },
  "4425": {
    "SetÃºbal (Escola Viso) - Mitrena": "4425_0_1.json",
    "SetÃºbal (Bairro Viso) - Mitrena": "4425_1_1.json",
    "Mitrena - SetÃºbal (Escola Viso)": "4425_0_2.json",
    "Mitrena - SetÃºbal (Bairro Viso)": "4425_1_2.json"
  },
  "4426": {
    "SetÃºbal (Bairro Viso) - SetÃºbal (CHEsetÃºbal)": "4426_0_1.json",
    "SetÃºbal (Escola Viso) - SetÃºbal (CHEsetÃºbal)": "4426_1_1.json",
    "SetÃºbal (Escola Viso) - SetÃºbal (ITS)": "4426_2_1.json",
    "SetÃºbal (CHEsetÃºbal) - SetÃºbal (Bairro Viso)": "4426_0_2.json",
    "SetÃºbal (CHEsetÃºbal) - SetÃºbal (Escola Viso)": "4426_1_2.json",
    "SetÃºbal (Bonfim) - SetÃºbal (Escola Viso)": "4426_3_2.json",
    "SetÃºbal (Mercado 2 de Abril) - SetÃºbal (Escola Viso)": "4426_4_2.json"
  },
  "4427": {
    "SetÃºbal (Bela Vista) - SetÃºbal (Mercado)": "4427_0_1.json",
    "SetÃºbal (Mercado) - SetÃºbal (Bela Vista)": "4427_0_2.json"
  },
  "4428": {
    "SetÃºbal (Casal Figueiras) - Vale Ana Gomes": "4428_0_1.json",
    "Vale Ana Gomes - SetÃºbal (Casal Figueiras)": "4428_0_2.json"
  },
  "4429": {
    "SetÃºbal (Centro SaÃºde) - SetÃºbal (Mercado)": "4429_0_1.json",
    "SetÃºbal (Mercado) - SetÃºbal (Centro SaÃºde)": "4429_0_2.json"
  },
  "4430": {
    "SetÃºbal (Hospital) - SetÃºbal (MontalvÃ£o)": "4430_0_1.json",
    "SetÃºbal (MontalvÃ£o) - SetÃºbal (Hospital)": "4430_0_2.json"
  },
  "4431": {
    "SetÃºbal (ITS) - SetÃºbal (Quinta Varzinha)": "4431_0_1.json",
    "SetÃºbal (Quinta Varzinha) - SetÃºbal (ITS)": "4431_0_2.json"
  },
  "4432": {
    "SetÃºbal (ITS) - Vale de Choupo": "4432_0_1.json",
    "Vale de Choupo - SetÃºbal (ITS)": "4432_0_2.json"
  },
  "4433": {
    "Alto Guerra - SetÃºbal (Casal Figueiras)": "4433_0_1.json",
    "SetÃºbal (Casal Figueiras) - Alto Guerra": "4433_0_2.json"
  },
  "4434": {
    "SetÃºbal (Mercado 2 de Abril) - SetÃºbal (R. Timor)": "4434_0_1.json",
    "Manteigadas - SetÃºbal (R. Timor)": "4434_1_1.json",
    "SetÃºbal (R. Timor) - SetÃºbal (Mercado 2 de Abril)": "4434_0_2.json",
    "SetÃºbal (R. Timor) - Manteigadas": "4434_1_2.json"
  },
  "4435": {
    "Biscainho - FaralhÃ£o": "4435_0_1.json",
    "FaralhÃ£o - Biscainho": "4435_0_2.json"
  },
  "4436": {
    "SetÃºbal (Mercado) - SetÃºbal (Av. Soeiro Pereira Gomes)": "4436_0_1.json",
    "SetÃºbal (Av. Soeiro Pereira Gomes) - SetÃºbal (Mercado)": "4436_0_2.json"
  },
  "4437": {
    "FaralhÃ£o - SetÃºbal (ITS)": "4437_0_1.json",
    "SetÃºbal (ITS) - FaralhÃ£o": "4437_0_2.json"
  },
  "4438": {
    "SetÃºbal (Monte Belo Norte) - SetÃºbal (Saboaria)": "4438_0_1.json",
    "SetÃºbal (Monte Belo Norte) - SetÃºbal (Saboaria) via Centro SaÃºde SÃ£o SebastiÃ£o": "4438_1_1.json",
    "SetÃºbal (Saboaria) - SetÃºbal (Monte Belo Norte)": "4438_0_2.json",
    "SetÃºbal (Saboaria) via Centro SaÃºde SÃ£o SebastiÃ£o - SetÃºbal (Monte Belo Norte)": "4438_1_2.json"
  },
  "4439": {
    "Praias do Sado - SetÃºbal (ITS)": "4439_0_1.json",
    "SetÃºbal (ITS) - Praias do Sado": "4439_0_2.json"
  },
  "4440": {
    "SetÃºbal (Monte Belo Norte) - SetÃºbal (Saboaria) via Alegro": "4440_0_1.json",
    "SetÃºbal (Saboaria) via Alegro - SetÃºbal (Monte Belo Norte)": "4440_0_2.json"
  },
  "4441": {
    "SetÃºbal (Saboaria) - SetÃºbal (Vale Cobro)": "4441_0_1.json",
    "SetÃºbal (Vale Cobro) - SetÃºbal (Saboaria)": "4441_0_2.json"
  },
  "4442": {
    "Praias do Sado (EstaÃ§Ã£o) - SetÃºbal (Bela Vista)": "4442_0_1.json",
    "Praias do Sado (EstaÃ§Ã£o) - SetÃºbal (Escola Luisa Todi)": "4442_1_1.json",
    "SetÃºbal (Bela Vista) - Praias do Sado (EstaÃ§Ã£o)": "4442_0_2.json"
  },
  "4443": {
    "SetÃºbal (PolitÃ©cnico) - Praias do Sado": "4443_0_1.json",
    "Praias do Sado - SetÃºbal (PolitÃ©cnico)": "4443_0_2.json"
  },
  "4451": {
    "Mitrena (Lisnave) - SetÃºbal (ITS)": "4451_0_1.json",
    "SetÃºbal (ITS) - Mitrena (Lisnave)": "4451_0_2.json"
  },
  "4452": {
    "Mitrena (Portucel) - SetÃºbal (ITS)": "4452_0_1.json",
    "Mitrena (Lisnave) - SetÃºbal (ITS)": "4452_1_1.json",
    "SetÃºbal (ITS) - Mitrena (Portucel)": "4452_0_2.json",
    "SetÃºbal (ITS) - Mitrena (Lisnave)": "4452_1_2.json"
  },
  "4453": {
    "Mitrena (Portucel) - SetÃºbal (ITS) via Estrada GraÃ§a": "4453_0_1.json",
    "SetÃºbal (ITS) via Estrada GraÃ§a - Mitrena (Portucel)": "4453_0_2.json"
  },
  "4460": {},
  "4470": {
    "Brejos AzeitÃ£o - Praia do Creiro": "4470_0_1.json",
    "Praia do Creiro - Brejos AzeitÃ£o": "4470_0_2.json"
  },
  "4471": {
    "Praia Albarquel | Circular": "4471_0_3.json"
  },
  "4472": {
    "Praia do Creiro - SetÃºbal (ITS)": "4472_0_1.json",
    "Praia da Figueirinha - SetÃºbal (ITS)": "4472_1_1.json",
    "SetÃºbal (ITS) - Praia do Creiro": "4472_0_2.json",
    "SetÃºbal (ITS) - Praia da Figueirinha": "4472_1_2.json"
  },
  "4474": {
    "Figueirinha - SetÃºbal (Alegro)": "4474_0_1.json",
    "SetÃºbal (Alegro) - Figueirinha": "4474_0_2.json"
  },
  "4475": {
    "Portinho da ArrÃ¡bida - Viso": "4475_0_1.json",
    "Viso - Portinho da ArrÃ¡bida": "4475_0_2.json",
    "Viso - Rasca": "4475_1_2.json"
  },
  "4476": {
    "Praias ArrÃ¡bida | Circular": "4476_0_3.json"
  },
  "4501": {
    "Alcochete - Montijo (Terminal Fluvial)": "4501_0_1.json",
    "Alcochete - Montijo (Terminal RodoviÃ¡rio)": "4501_1_1.json",
    "Montijo (Terminal Fluvial) - Alcochete": "4501_0_2.json",
    "Montijo (Terminal RodoviÃ¡rio) - Alcochete": "4501_1_2.json"
  },
  "4502": {
    "Alcochete - Passil": "4502_0_1.json",
    "Passil - Alcochete": "4502_0_2.json"
  },
  "4503": {
    "Atalaia - Jardia": "4503_0_1.json",
    "Jardia - Atalaia": "4503_0_2.json"
  },
  "4504": {
    "Montijo (Terminal Fluvial) - Passil": "4504_0_1.json",
    "Passil - Montijo (Terminal Fluvial)": "4504_0_2.json"
  },
  "4510": {
    "Alcochete (Freeport) - Montijo (Terminal RodoviÃ¡rio)": "4510_0_1.json",
    "Montijo (Terminal RodoviÃ¡rio) - Alcochete (Freeport)": "4510_0_2.json"
  },
  "4511": {
    "Alcochete (Freeport) - Montijo (Terminal RodoviÃ¡rio)": "4511_0_1.json",
    "Montijo (Terminal RodoviÃ¡rio) - Alcochete (Freeport)": "4511_0_2.json"
  },
  "4512": {
    "Alcochete (Freeport) - SetÃºbal (ITS) via Alto Estanqueiro": "4512_0_1.json",
    "Alcochete (Freeport) - SetÃºbal (ITS)": "4512_1_1.json",
    "SetÃºbal (ITS) via Alto Estanqueiro - Alcochete (Freeport)": "4512_0_2.json",
    "SetÃºbal (ITS) - Alcochete (Freeport)": "4512_1_2.json"
  },
  "4513": {
    "Alcochete (Freeport) - Pinhal Novo": "4513_0_1.json",
    "Alcochete (Freeport) - Pinhal Novo via Montijo (Ã�rea Comercial)": "4513_1_1.json",
    "Pinhal Novo - Alcochete (Freeport)": "4513_0_2.json"
  },
  "4514": {
    "Canha - Montijo (Terminal RodoviÃ¡rio) via PegÃµes": "4514_0_1.json",
    "Canha - Montijo (Terminal RodoviÃ¡rio)": "4514_1_1.json",
    "Montijo (Terminal RodoviÃ¡rio) via PegÃµes - Canha": "4514_0_2.json",
    "Loja Nova - Canha": "4514_2_2.json",
    "Montijo (Terminal RodoviÃ¡rio) - Canha": "4514_3_2.json"
  },
  "4515": {
    "Montijo (Terminal RodoviÃ¡rio) - PegÃµes": "4515_0_1.json",
    "PegÃµes - Montijo (Terminal RodoviÃ¡rio)": "4515_0_2.json",
    "PegÃµes - Loja Nova": "4515_1_2.json"
  },
  "4516": {
    "Montijo (Terminal RodoviÃ¡rio) - Rio Frio": "4516_0_1.json",
    "Montijo (Terminal RodoviÃ¡rio) - Charnequinha": "4516_1_1.json",
    "Rio Frio - Montijo (Terminal RodoviÃ¡rio)": "4516_0_2.json"
  },
  "4517": {
    "SetÃºbal (ITS) - Montijo (Terminal RodoviÃ¡rio)": "4517_0_2.json"
  },
  "4520": {
    "Faias - PegÃµes": "4520_0_1.json",
    "PegÃµes - Faias": "4520_0_2.json"
  },
  "4521": {
    "Faias - Pinhal Novo": "4521_0_1.json",
    "Lau - Pinhal Novo": "4521_1_1.json",
    "Pinhal Novo - Faias": "4521_0_2.json"
  },
  "4522": {
    "Faias - PoceirÃ£o": "4522_0_1.json",
    "PoceirÃ£o - Faias": "4522_0_2.json"
  },
  "4523": {
    "Pinhal Novo - Montijo (Terminal RodoviÃ¡rio)": "4523_0_2.json"
  },
  "4524": {
    "Palmela (Terminal) - PegÃµes": "4524_0_1.json",
    "Palmela (Terminal) - Loja Nova": "4524_1_1.json",
    "PegÃµes - Palmela (Terminal)": "4524_0_2.json"
  },
  "4530": {
    "Bairro Vila Morena - Pinhal Novo": "4530_0_1.json",
    "Pinhal Novo - Bairro Vila Morena": "4530_0_2.json"
  },
  "4531": {
    "Moita - Palmela (Terminal)": "4531_0_1.json",
    "Palmela (Terminal) - Moita": "4531_0_2.json"
  },
  "4532": {
    "Moita - Quatro Marcos": "4532_0_1.json",
    "Quatro Marcos - Moita": "4532_0_2.json"
  },
  "4540": {
    "Ã�guas de Moura - SetÃºbal (ITS)": "4540_0_1.json",
    "Ã�guas de Moura - SetÃºbal (ITS) via GÃ¢mbia": "4540_1_1.json",
    "SetÃºbal (ITS) - Ã�guas de Moura": "4540_0_2.json",
    "SetÃºbal (ITS) via GÃ¢mbia - Ã�guas de Moura": "4540_1_2.json"
  },
  "4541": {
    "Algeruz - SetÃºbal (Av. LuÃ­sa Todi)": "4541_0_1.json",
    "SetÃºbal (Av. LuÃ­sa Todi) - Algeruz": "4541_0_2.json"
  },
  "4542": {
    "Algeruz - SetÃºbal (ITS)": "4542_0_1.json",
    "SetÃºbal (ITS) - Algeruz": "4542_0_2.json"
  },
  "4543": {
    "Algeruz - SetÃºbal (ITS) via PoÃ§oilos": "4543_0_1.json",
    "SetÃºbal (ITS) via PoÃ§oilos - Algeruz": "4543_0_2.json"
  },
  "4544": {
    "Bairro MargaÃ§a - SetÃºbal (ITS)": "4544_0_1.json",
    "Bairro MargaÃ§a - SetÃºbal (ITS) via GÃ¢mbia": "4544_1_1.json",
    "SetÃºbal (ITS) - Bairro MargaÃ§a": "4544_0_2.json",
    "SetÃºbal (Av. 5 de Outubro) - Bairro MargaÃ§a": "4544_2_2.json"
  },
  "4545": {
    "Biscainho - SetÃºbal (Bela Vista)": "4545_0_1.json",
    "Biscainho - SetÃºbal (Bela Vista) via Vale de Rosa": "4545_1_1.json",
    "SetÃºbal (Bela Vista) - Biscainho": "4545_0_2.json",
    "SetÃºbal (Bela Vista) via Vale de Rosa - Biscainho": "4545_1_2.json"
  },
  "4546": {
    "Biscainho - SetÃºbal (ITS)": "4546_0_1.json",
    "SetÃºbal (ITS) - Biscainho": "4546_0_2.json"
  },
  "4547": {
    "Cabanas - SetÃºbal (ITS)": "4547_0_1.json",
    "SetÃºbal (ITS) - Cabanas": "4547_0_2.json"
  },
  "4548": {
    "LagameÃ§as - SetÃºbal (ITS)": "4548_0_1.json",
    "SetÃºbal (ITS) - LagameÃ§as": "4548_0_2.json"
  },
  "4549": {
    "Palmela (Terminal) - SetÃºbal (ITS)": "4549_0_1.json",
    "SetÃºbal (ITS) - Palmela (Terminal)": "4549_0_2.json"
  },
  "4550": {
    "Palmela (Terminal) - Vila Nogueira de AzeitÃ£o": "4550_0_1.json",
    "Vila Nogueira de AzeitÃ£o - Palmela (Terminal)": "4550_0_2.json",
    "Quinta do Anjo - Palmela (Terminal)": "4550_1_2.json"
  },
  "4551": {
    "Palmela (USF) - SetÃºbal (Av. LuÃ­sa Todi)": "4551_0_1.json",
    "SetÃºbal (Av. LuÃ­sa Todi) - Palmela (USF)": "4551_0_2.json"
  },
  "4560": {
    "Cabanas - Vila Nogueira de AzeitÃ£o": "4560_0_1.json",
    "Brejos de AzeitÃ£o - Vila Nogueira de AzeitÃ£o": "4560_1_1.json",
    "Vila Nogueira de AzeitÃ£o - Cabanas": "4560_0_2.json",
    "Vila Nogueira de AzeitÃ£o - Brejos de AzeitÃ£o": "4560_1_2.json"
  },
  "4561": {
    "Cabanas - Vila Nogueira de AzeitÃ£o via Quinta do PicÃ£o": "4561_0_1.json",
    "Vila Nogueira de AzeitÃ£o via Quinta do PicÃ£o - Cabanas": "4561_0_2.json"
  },
  "4562": {
    "SetÃºbal (ITS) - Vila Nogueira de AzeitÃ£o via Palmela (EstaÃ§Ã£o)": "4562_0_1.json",
    "SetÃºbal (ITS) - Vendas de AzeitÃ£o": "4562_1_1.json",
    "SetÃºbal (ITS) - Vila Nogueira de AzeitÃ£o via SetÃºbal (Alegro) e Palmela (EstaÃ§Ã£o)": "4562_2_1.json",
    "SetÃºbal (ITS) - AzeitÃ£o (EB 2 3) via Palmela (EstaÃ§Ã£o)": "4562_3_1.json",
    "SetÃºbal (ITS) - AzeitÃ£o (EB 2 3) via SetÃºbal (Alegro) e Palmela (EstaÃ§Ã£o)": "4562_4_1.json",
    "SetÃºbal (ITS) - AzeitÃ£o (EB 2 3) via Palmela (EstaÃ§Ã£o) e Centro SaÃºde Quinta do Anjo": "4562_5_1.json",
    "Vila Nogueira de AzeitÃ£o via SetÃºbal (Alegro) e Palmela (EstaÃ§Ã£o) - SetÃºbal (ITS)": "4562_2_2.json",
    "AzeitÃ£o (EB 2 3) via Palmela (EstaÃ§Ã£o) - SetÃºbal (ITS)": "4562_3_2.json",
    "AzeitÃ£o (EB 2 3) via Palmela (EstaÃ§Ã£o) e Centro SaÃºde Quinta do Anjo - SetÃºbal (ITS)": "4562_5_2.json",
    "Vila Nogueira de AzeitÃ£o via Palmela (EstaÃ§Ã£o) - SetÃºbal (ITS)": "4562_7_2.json"
  },
  "4600": {
    "Alcochete (Freeport) - Barreiro (Terminal)": "4600_0_1.json",
    "Alcochete (Freeport) - Barreiro (Terminal) via Montijo (Ã�rea Comercial)": "4600_1_1.json",
    "Barreiro (Terminal) - Alcochete (Freeport)": "4600_0_2.json",
    "Barreiro (Terminal) via Montijo (Ã�rea Comercial) - Alcochete (Freeport)": "4600_1_2.json"
  },
  "4601": {
    "Barreiro (Terminal) - Montijo (Terminal RodoviÃ¡rio)": "4601_0_1.json",
    "Barreiro (Terminal) - Montijo (Terminal RodoviÃ¡rio) via Montijo (Ã�rea Comercial)": "4601_1_1.json",
    "Montijo (Terminal RodoviÃ¡rio) - Barreiro (Terminal)": "4601_0_2.json"
  },
  "4602": {
    "Alhos Vedros (EstaÃ§Ã£o) - Barreiro (Terminal)": "4602_0_1.json",
    "Barreiro (Terminal) - Alhos Vedros (EstaÃ§Ã£o)": "4602_0_2.json"
  },
  "4603": {
    "Barreiro (Terminal) - ChÃ£o Duro": "4603_0_1.json",
    "ChÃ£o Duro - Barreiro (Terminal)": "4603_0_2.json"
  },
  "4604": {
    "Barreiro (Terminal) - Moita (Escola Fragata do Tejo)": "4604_0_1.json",
    "Moita (Escola Fragata do Tejo) - Barreiro (Terminal)": "4604_0_2.json"
  },
  "4605": {
    "Lavradio - Pinhal do Forno": "4605_0_1.json",
    "Pinhal do Forno - Lavradio": "4605_0_2.json"
  },
  "4610": {
    "Bairro dos Marinheiros - Barreiro (Terminal)": "4610_0_1.json",
    "Barreiro (Terminal) - Bairro dos Marinheiros": "4610_0_2.json"
  },
  "4611": {
    "Penalva - Moita (Esc. SecundÃ¡ria)": "4611_0_1.json",
    "Moita (Esc. SecundÃ¡ria) - Penalva": "4611_0_2.json"
  },
  "4612": {
    "Bairro dos Marinheiros - Palmela (Terminal)": "4612_0_1.json",
    "Palmela (Terminal) - Bairro dos Marinheiros": "4612_0_2.json"
  },
  "4620": {
    "Moita - Paio Pires": "4620_0_1.json",
    "Paio Pires - Moita": "4620_0_2.json"
  },
  "4621": {
    "Moita - Seixal (Terminal Fluvial)": "4621_0_1.json",
    "Seixal (Terminal Fluvial) - Moita": "4621_0_2.json"
  },
  "4630": {
    "Corroios (EstaÃ§Ã£o) - SetÃºbal (ITS)": "4630_0_1.json",
    "SetÃºbal (ITS) - Corroios (EstaÃ§Ã£o)": "4630_0_2.json"
  },
  "4631": {
    "Fogueteiro (EstaÃ§Ã£o) - SetÃºbal (ITS)": "4631_0_1.json",
    "Lisboa (Sete Rios) - SetÃºbal (ITS)": "4631_1_1.json",
    "SetÃºbal (ITS) - Fogueteiro (EstaÃ§Ã£o)": "4631_0_2.json",
    "SetÃºbal (ITS) - Lisboa (Sete Rios)": "4631_1_2.json"
  },
  "4640": {
    "Casais da Serra - Vila Nogueira de AzeitÃ£o": "4640_0_1.json",
    "Vila Nogueira de AzeitÃ£o - Casais da Serra": "4640_0_2.json"
  },
  "4641": {
    "Quinta do Conde - SetÃºbal (ITS)": "4641_0_1.json",
    "SetÃºbal (ITS) - Quinta do Conde": "4641_0_2.json"
  },
  "4642": {
    "Sesimbra (Terminal) - SetÃºbal (Hospital)": "4642_0_1.json",
    "Sesimbra (Terminal) - SetÃºbal (Hospital) via R. Vinha do Sardinha": "4642_1_1.json",
    "SetÃºbal (Hospital) - Sesimbra (Terminal)": "4642_0_2.json",
    "SetÃºbal (Hospital) via R. Vinha do Sardinha - Sesimbra (Terminal)": "4642_1_2.json"
  },
  "4643": {
    "Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)": "4643_0_1.json",
    "Sesimbra (Terminal) - Montijo (Av. Inf. D. Henrique)": "4643_0_2.json"
  },
  "4701": {
    "Lisboa (Oriente) - Vale da Amoreira": "4701_0_1.json",
    "Vale da Amoreira - Lisboa (Oriente)": "4701_0_2.json",
    "Moita - Lisboa (Oriente)": "4701_1_2.json"
  },
  "4702": {
    "Lisboa (Oriente) - Valbom": "4702_0_1.json",
    "Valbom - Lisboa (Oriente)": "4702_0_2.json"
  },
  "4703": {
    "Lisboa (Oriente) - Montijo (Terminal RodoviÃ¡rio) via Alcochete e Samouco": "4703_0_1.json",
    "Montijo (Terminal RodoviÃ¡rio) via Alcochete e Samouco - Lisboa (Oriente)": "4703_0_2.json"
  },
  "4704": {
    "Atalaia - Lisboa (Oriente)": "4704_0_1.json",
    "Lisboa (Oriente) - Atalaia": "4704_0_2.json"
  },
  "4705": {
    "Lisboa (Oriente) - Samouco": "4705_0_1.json",
    "Samouco - Lisboa (Oriente)": "4705_0_2.json"
  },
  "4706": {
    "SÃ£o Francisco - Lisboa (Oriente)": "4706_0_1.json",
    "Lisboa (Oriente) - SÃ£o Francisco": "4706_0_2.json"
  },
  "4707": {
    "Lisboa (Oriente) - Montijo (Terminal RodoviÃ¡rio)": "4707_0_1.json",
    "Montijo (Terminal RodoviÃ¡rio) - Lisboa (Oriente)": "4707_0_2.json"
  },
  "4710": {
    "Lisboa (Oriente) - Palmela (Terminal)": "4710_0_1.json",
    "Palmela (Terminal) - Lisboa (Oriente)": "4710_0_2.json"
  },
  "4711": {
    "Pinhal Novo - Lisboa (Oriente)": "4711_0_2.json"
  },
  "4715": {
    "Lisboa (Oriente) - SetÃºbal (ITS) via Pinhal Novo": "4715_1_1.json",
    "SetÃºbal (ITS) via Pinhal Novo - Lisboa (Oriente)": "4715_1_2.json"
  },
  "4720": {
    "Lisboa (Oriente) - SetÃºbal (ITS)": "4720_0_1.json",
    "SetÃºbal (ITS) - Lisboa (Oriente)": "4720_0_2.json"
  },
  "4725": {
    "Lisboa (Sete Rios) - SetÃºbal (ITS)": "4725_0_1.json",
    "SetÃºbal (ITS) - Lisboa (Sete Rios)": "4725_0_2.json"
  },
  "4901": {
    "Landeira - SetÃºbal (ITS)": "4901_0_1.json",
    "Landeira - SetÃºbal (ITS) via GÃ¢mbia": "4901_1_1.json",
    "SetÃºbal (ITS) - Landeira": "4901_0_2.json",
    "SetÃºbal (ITS) via GÃ¢mbia - Landeira": "4901_1_2.json"
  },
  "4902": {
    "Landeira - PegÃµes": "4902_0_1.json",
    "PegÃµes - Landeira": "4902_0_2.json"
  },
  "4905": {
    "Faias - Vendas Novas": "4905_0_1.json",
    "Vendas Novas - Faias": "4905_0_2.json"
  },
  "4906": {
    "SetÃºbal (ITS) - Vendas Novas via Landeira": "4906_0_1.json",
    "Vendas Novas via Landeira - SetÃºbal (ITS)": "4906_0_2.json",
    "Vendas Novas - SetÃºbal (ITS)": "4906_1_2.json"
  }
};
const horarios_pdf = {
  "3003_0_1": "Linha_3003_020088_G_2022-06-23-16-23.pdf",
  "3003_0_2": "Linha_3003_020113_R_2022-06-23-16-24.pdf",
  "3004_0_1": "Linha_3004_020088_G_2022-06-23-16-23.pdf",
  "3004_0_2": "Linha_3004_020181_R_2022-06-23-16-24.pdf",
  "3007_0_2": "Linha_3007_020113_R_2022-06-23-16-25.pdf",
  "3007_0_1": "Linha_3007_020254_G_2022-06-23-16-25.pdf",
  "3008_0_1": "Linha_3008_020298_G_2022-06-23-16-24.pdf",
  "3008_0_2": "Linha_3008_020884_R_2022-06-23-16-25.pdf",
  "3009_0_1": "Linha_3009_020363_G_2022-06-23-16-25.pdf",
  "3009_0_2": "Linha_3009_020385_R_2022-06-23-16-25.pdf",
  "3010_0_2": "Linha_3010_020179_R_2022-06-23-16-25.pdf",
  "3010_0_1": "Linha_3010_020403_G_2022-06-23-16-25.pdf",
  "3011_0_1": "Linha_3011_020429_G_2022-06-23-16-25.pdf",
  "3011_0_2": "Linha_3011_020444_R_2022-06-23-16-26.pdf",
  "3012_0_1": "Linha_3012_020403_G_2022-06-23-16-25.pdf",
  "3012_0_2": "Linha_3012_020800_R_2022-06-23-16-26.pdf",
  "3013_0_2": "Linha_3013_020313_R_2022-06-23-16-26.pdf",
  "3013_0_1": "Linha_3013_020489_G_2022-06-23-16-26.pdf",
  "3014_0_1": "Linha_3014_020363_G_2022-06-23-16-26.pdf",
  "3014_0_2": "Linha_3014_020549_R_2022-06-23-16-26.pdf",
  "3015_0_1": "Linha_3015_020179_G_2022-06-23-16-26.pdf",
  "3015_0_2": "Linha_3015_020569_R_2022-06-23-16-27.pdf",
  "3017_0_2": "Linha_3017_020073_R_2022-06-23-16-27.pdf",
  "3017_0_1": "Linha_3017_020179_G_2022-06-23-16-27.pdf",
  "3019_0_1": "Linha_3019_020179_G_2022-06-23-16-27.pdf",
  "3019_0_2": "Linha_3019_020385_R_2022-06-23-16-27.pdf",
  "3021_0_2": "Linha_3021_020317_R_2022-06-23-22-17.pdf",
  "3021_0_1": "Linha_3021_020443_G_2022-06-23-22-17.pdf",
  "3022_0_2": "Linha_3022_020291_R_2022-06-23-16-29.pdf",
  "3022_0_1": "Linha_3022_020637_G_2022-06-23-16-29.pdf",
  "3024_0_2": "Linha_3024_020073_R_2022-06-23-22-16.pdf",
  "3024_0_1": "Linha_3024_020637_G_2022-06-23-22-16.pdf",
  "3026_0_1": "Linha_3026_020533_G_2022-06-30-18-05.pdf",
  "3026_0_2": "Linha_3026_020758_R_2022-06-30-18-06.pdf",
  "3027_0_2": "Linha_3027_020617_R_2022-06-30-16-52.pdf",
  "3027_0_1": "Linha_3027_020755_G_2022-06-30-16-52.pdf",
  "3030_0_2": "Linha_3030_020317_R_2022-06-23-16-30.pdf",
  "3030_0_1": "Linha_3030_020800_G_2022-06-23-16-30.pdf",
  "3034_0_2": "Linha_3034_020835_R_2022-06-23-16-31.pdf",
  "3034_0_1": "Linha_3034_020839_G_2022-06-23-16-31.pdf",
  "3035_0_1": "Linha_3035_020073_G_2022-06-23-16-31.pdf",
  "3035_0_2": "Linha_3035_020835_R_2022-06-23-16-32.pdf",
  "3036_0_1": "Linha_3036_020073_G_2022-06-23-16-31.pdf",
  "3036_0_2": "Linha_3036_020863_R_2022-06-23-16-32.pdf",
  "3101_0_1": "Linha_3101_140143_G_2022-06-23-16-32.pdf",
  "3101_0_2": "Linha_3101_140167_R_2022-06-23-16-33.pdf",
  "3103_0_1": "Linha_3103_140089_G_2022-06-23-16-33.pdf",
  "3103_0_2": "Linha_3103_140336_R_2022-06-23-16-33.pdf",
  "3104_0_1": "Linha_3104_140089_G_2022-06-23-16-32.pdf",
  "3104_0_2": "Linha_3104_140387_R_2022-06-23-16-33.pdf",
  "3105_0_2": "Linha_3105_140039_R_2022-06-23-16-33.pdf",
  "3105_0_1": "Linha_3105_140391_G_2022-06-23-16-34.pdf",
  "3107_0_2": "Linha_3107_140416_R_2022-06-23-16-33.pdf",
  "3107_0_1": "Linha_3107_140469_G_2022-06-23-16-33.pdf",
  "3108_0_2": "Linha_3108_140167_R_2022-06-23-16-34.pdf",
  "3108_0_1": "Linha_3108_140511_G_2022-06-23-16-34.pdf",
  "3109_0_1": "Linha_3109_140139_G_2022-06-23-16-34.pdf",
  "3109_0_2": "Linha_3109_140535_R_2022-06-23-16-34.pdf",
  "3110_0_1": "Linha_3110_140537_G_2022-06-23-16-34.pdf",
  "3110_0_2": "Linha_3110_140554_R_2022-06-23-16-34.pdf",
  "3111_0_2": "Linha_3111_140074_R_2022-06-30-18-07.pdf",
  "3111_0_1": "Linha_3111_140555_G_2022-06-30-18-08.pdf",
  "3112_0_1": "Linha_3112_140039_G_2022-06-23-16-35.pdf",
  "3112_0_2": "Linha_3112_140074_R_2022-06-23-16-36.pdf",
  "3113_0_1": "Linha_3113_140039_G_2022-06-23-16-38.pdf",
  "3113_0_2": "Linha_3113_140074_R_2022-06-23-16-38.pdf",
  "3114_0_1": "Linha_3114_140167_G_2022-06-23-16-35.pdf",
  "3114_0_2": "Linha_3114_140276_R_2022-06-23-16-36.pdf",
  "3119_0_2": "Linha_3119_140074_R_2022-06-23-16-39.pdf",
  "3119_0_1": "Linha_3119_140193_G_2022-06-23-16-39.pdf",
  "3120_0_2": "Linha_3120_140074_R_2022-06-23-16-39.pdf",
  "3120_0_1": "Linha_3120_140554_G_2022-06-23-16-39.pdf",
  "3121_0_1": "Linha_3121_140671_G_2022-06-23-16-39.pdf",
  "3201_0_2": "Linha_3201_150052_R_2022-06-23-16-40.pdf",
  "3201_0_1": "Linha_3201_150101_G_2022-06-23-16-40.pdf",
  "3202_0_2": "Linha_3202_150052_R_2022-06-23-16-40.pdf",
  "3202_0_1": "Linha_3202_150116_G_2022-06-23-16-40.pdf",
  "3203_0_2": "Linha_3203_150052_R_2022-06-23-16-40.pdf",
  "3203_0_1": "Linha_3203_150165_G_2022-06-23-16-40.pdf",
  "3204_0_2": "Linha_3204_150052_R_2022-06-23-16-40.pdf",
  "3204_0_1": "Linha_3204_150165_G_2022-06-23-16-40.pdf",
  "3205_0_2": "Linha_3205_150052_R_2022-06-23-16-40.pdf",
  "3205_0_1": "Linha_3205_150213_G_2022-06-23-16-40.pdf",
  "3206_0_2": "Linha_3206_150052_R_2022-06-23-16-40.pdf",
  "3206_0_1": "Linha_3206_150215_G_2022-06-23-16-40.pdf",
  "3207_0_2": "Linha_3207_150052_R_2022-06-23-16-41.pdf",
  "3207_0_1": "Linha_3207_150234_G_2022-06-23-16-41.pdf",
  "3208_0_2": "Linha_3208_150052_R_2022-06-23-16-41.pdf",
  "3208_0_1": "Linha_3208_150289_G_2022-06-23-16-41.pdf",
  "3209_0_2": "Linha_3209_150052_R_2022-06-23-16-41.pdf",
  "3209_0_1": "Linha_3209_150291_G_2022-06-23-16-41.pdf",
  "3210_0_2": "Linha_3210_150052_R_2022-06-23-16-41.pdf",
  "3210_0_1": "Linha_3210_150301_G_2022-06-23-16-41.pdf",
  "3211_0_2": "Linha_3211_150052_R_2022-06-23-21-01.pdf",
  "3211_0_1": "Linha_3211_150301_G_2022-06-23-21-01.pdf",
  "3212_0_2": "Linha_3212_150052_R_2022-06-23-16-41.pdf",
  "3212_0_1": "Linha_3212_150341_G_2022-06-23-16-41.pdf",
  "3213_0_1": "Linha_3213_150051_G_2022-06-23-16-41.pdf",
  "3214_0_1": "Linha_3214_150259_G_2022-06-23-16-41.pdf",
  "3214_0_2": "Linha_3214_150407_R_2022-06-23-16-42.pdf",
  "3215_0_2": "Linha_3215_150259_R_2022-06-23-16-42.pdf",
  "3215_0_1": "Linha_3215_150292_G_2022-06-23-16-42.pdf",
  "3216_0_2": "Linha_3216_150259_R_2022-06-23-16-42.pdf",
  "3216_0_1": "Linha_3216_150413_G_2022-06-23-16-42.pdf",
  "3217_0_1": "Linha_3217_150170_G_2022-06-23-16-42.pdf",
  "3217_0_2": "Linha_3217_150259_R_2022-06-23-16-42.pdf",
  "3218_0_2": "Linha_3218_150051_R_2022-06-30-18-10.pdf",
  "3218_0_1": "Linha_3218_150415_G_2022-06-30-18-09.pdf",
  "3220_0_1": "Linha_3220_150453_G_2022-06-23-16-43.pdf",
  "3221_0_2": "Linha_3221_150052_R_2022-06-23-16-43.pdf",
  "3221_0_1": "Linha_3221_150242_G_2022-06-23-16-43.pdf",
  "3222_0_2": "Linha_3222_150483_R_2022-06-23-16-43.pdf",
  "3222_0_1": "Linha_3222_150484_G_2022-06-23-16-43.pdf",
  "3502_0_1": "Linha_3502_020088_G_2022-06-23-16-43.pdf",
  "3502_0_2": "Linha_3502_140021_R_2022-06-23-16-44.pdf",
  "3506_0_1": "Linha_3506_020113_G_2022-06-23-16-47.pdf",
  "3506_0_2": "Linha_3506_140089_R_2022-06-23-16-48.pdf",
  "3507_0_1": "Linha_3507_020429_G_2022-06-23-16-47.pdf",
  "3507_0_2": "Linha_3507_140731_R_2022-06-23-16-48.pdf",
  "3508_0_1": "Linha_3508_020955_G_2022-06-23-16-48.pdf",
  "3508_0_2": "Linha_3508_140021_R_2022-06-23-16-48.pdf",
  "3509_0_1": "Linha_3509_020955_G_2022-06-24-00-05.pdf",
  "3509_0_2": "Linha_3509_140021_R_2022-06-24-00-05.pdf",
  "3510_0_1": "Linha_3510_020959_G_2022-06-23-16-47.pdf",
  "3510_0_2": "Linha_3510_020967_R_2022-06-23-16-48.pdf",
  "3511_0_1": "Linha_3511_020973_G_2022-06-23-16-48.pdf",
  "3511_0_2": "Linha_3511_020977_R_2022-06-23-16-48.pdf",
  "3512_0_1": "Linha_3512_020973_G_2022-06-23-16-49.pdf",
  "3512_0_2": "Linha_3512_140763_R_2022-06-23-16-49.pdf",
  "3513_0_1": "Linha_3513_020959_G_2022-06-30-17-16.pdf",
  "3513_0_2": "Linha_3513_140637_R_2022-06-30-17-17.pdf",
  "3514_0_1": "Linha_3514_020429_G_2022-06-23-16-49.pdf",
  "3514_0_2": "Linha_3514_140387_R_2022-06-23-16-49.pdf",
  "3515_0_2": "Linha_3515_020967_R_2022-06-23-16-49.pdf",
  "3515_0_1": "Linha_3515_140089_G_2022-06-23-16-49.pdf",
  "3516_0_1": "Linha_3516_020187_G_2022-06-23-16-50.pdf",
  "3516_0_2": "Linha_3516_140089_R_2022-06-23-16-50.pdf",
  "3517_0_1": "Linha_3517_020981_G_2022-06-23-16-50.pdf",
  "3517_0_2": "Linha_3517_140089_R_2022-06-23-16-50.pdf",
  "3518_0_2": "Linha_3518_020717_R_2022-06-23-16-50.pdf",
  "3518_0_1": "Linha_3518_140089_G_2022-06-23-16-50.pdf",
  "3519_0_1": "Linha_3519_020005_G_2022-06-23-16-51.pdf",
  "3519_0_2": "Linha_3519_140089_R_2022-06-23-16-51.pdf",
  "3520_0_1": "Linha_3520_020992_G_2022-06-23-16-51.pdf",
  "3520_0_2": "Linha_3520_140755_R_2022-06-23-16-52.pdf",
  "3521_0_2": "Linha_3521_020800_R_2022-06-23-16-52.pdf",
  "3521_0_1": "Linha_3521_140583_G_2022-06-23-16-52.pdf",
  "3522_0_1": "Linha_3522_020800_G_2022-06-23-22-59.pdf",
  "3522_0_2": "Linha_3522_140021_R_2022-06-23-23-00.pdf",
  "3523_0_1": "Linha_3523_020800_G_2022-06-23-23-00.pdf",
  "3523_0_2": "Linha_3523_140276_R_2022-06-23-23-01.pdf",
  "3524_0_1": "Linha_3524_020756_G_2022-06-23-16-52.pdf",
  "3524_0_2": "Linha_3524_140731_R_2022-06-23-16-52.pdf",
  "3525_0_1": "Linha_3525_020291_G_2022-06-23-16-52.pdf",
  "3525_0_2": "Linha_3525_140711_R_2022-06-23-16-52.pdf",
  "3526_0_1": "Linha_3526_020696_G_2022-06-23-16-52.pdf",
  "3526_0_2": "Linha_3526_020977_R_2022-06-23-16-53.pdf",
  "3527_0_1": "Linha_3527_020317_G_2022-06-30-18-12.pdf",
  "3527_0_2": "Linha_3527_140787_R_2022-06-30-18-13.pdf",
  "3528_0_1": "Linha_3528_020317_G_2022-06-23-23-01.pdf",
  "3528_0_2": "Linha_3528_140021_R_2022-06-23-23-02.pdf",
  "3535_0_1": "Linha_3535_021007_G_2022-06-23-23-02.pdf",
  "3535_0_2": "Linha_3535_021007_R_2022-06-23-23-02.pdf",
  "3536_0_1": "Linha_3536_021009_G_2022-06-30-13-16.pdf",
  "3536_0_2": "Linha_3536_150051_R_2022-06-30-13-17.pdf",
  "3543_0_2": "Linha_3543_150533_R_2022-06-23-16-53.pdf",
  "3545_0_1": "Linha_3545_140089_G_2022-06-23-16-54.pdf",
  "3545_0_2": "Linha_3545_150052_R_2022-06-23-16-54.pdf",
  "3546_0_1": "Linha_3546_140103_G_2022-06-23-23-04.pdf",
  "3546_0_2": "Linha_3546_150553_R_2022-06-23-23-04.pdf",
  "3547_0_1": "Linha_3547_140139_G_2022-06-24-00-15.pdf",
  "3547_0_2": "Linha_3547_150553_R_2022-06-24-00-15.pdf",
  "3548_0_1": "Linha_3548_140469_G_2022-06-23-16-54.pdf",
  "3548_0_2": "Linha_3548_150013_R_2022-06-23-16-54.pdf",
  "3549_0_1": "Linha_3549_150013_G_2022-06-23-23-39.pdf",
  "3549_0_2": "Linha_3549_150052_R_2022-06-23-23-39.pdf",
  "3601_0_2": "Linha_3601_020072_R_2022-06-23-16-54.pdf",
  "3601_0_1": "Linha_3601_040137_G_2022-06-23-16-54.pdf",
  "3605_0_1": "Linha_3605_020449_G_2022-06-24-00-17.pdf",
  "3605_0_2": "Linha_3605_160067_R_2022-06-24-00-17.pdf",
  "3615_0_1": "Linha_3615_040027_G_2022-06-23-16-54.pdf",
  "3615_0_2": "Linha_3615_140073_R_2022-06-23-16-55.pdf",
  "3620_0_1": "Linha_3620_140857_G_2022-06-23-16-55.pdf",
  "3620_0_2": "Linha_3620_150492_R_2022-06-23-16-55.pdf",
  "3626_0_1": "Linha_3626_140857_G_2022-06-30-18-14.pdf",
  "3626_0_2": "Linha_3626_160775_R_2022-06-30-18-15.pdf",
  "3635_0_1": "Linha_3635_140027_G_2022-06-23-16-56.pdf",
  "3635_0_2": "Linha_3635_150052_R_2022-06-23-16-56.pdf",
  "3640_0_1": "Linha_3640_150170_G_2022-06-30-14-08.pdf",
  "3640_0_2": "Linha_3640_160966_R_2022-06-30-14-08.pdf",
  "3641_0_1": "Linha_3641_150013_G_2022-06-23-16-56.pdf",
  "3641_0_2": "Linha_3641_150259_R_2022-06-23-16-56.pdf",
  "3702_0_1": "Linha_3702_020057_G_2022-06-23-16-57.pdf",
  "3702_0_2": "Linha_3702_060393_R_2022-06-23-16-57.pdf",
  "3703_0_1": "Linha_3703_020057_G_2022-06-23-16-57.pdf",
  "3703_0_2": "Linha_3703_060457_R_2022-06-23-16-58.pdf",
  "3704_0_1": "Linha_3704_020890_G_2022-06-23-16-57.pdf",
  "3704_0_2": "Linha_3704_060405_R_2022-06-23-16-57.pdf",
  "3707_0_1": "Linha_3707_020337_G_2022-06-23-17-04.pdf",
  "3707_0_2": "Linha_3707_060456_R_2022-06-23-17-04.pdf",
  "3709_0_1": "Linha_3709_021018_G_2022-06-23-17-04.pdf",
  "3709_0_2": "Linha_3709_060405_R_2022-06-23-17-04.pdf",
  "3710_0_1": "Linha_3710_020705_G_2022-06-23-17-05.pdf",
  "3710_0_2": "Linha_3710_060455_R_2022-06-23-17-05.pdf",
  "3711_0_1": "Linha_3711_020317_G_2022-06-23-17-04.pdf",
  "3711_0_2": "Linha_3711_060458_R_2022-06-23-17-05.pdf",
  "3715_0_1": "Linha_3715_060405_G_2022-06-23-17-05.pdf",
  "3715_0_2": "Linha_3715_140637_R_2022-06-23-17-05.pdf",
  "3716_0_1": "Linha_3716_060456_G_2022-06-23-17-05.pdf",
  "3716_0_2": "Linha_3716_140867_R_2022-06-23-17-05.pdf",
  "3717_0_1": "Linha_3717_060454_G_2022-06-23-17-05.pdf",
  "3717_0_2": "Linha_3717_140755_R_2022-06-23-17-06.pdf",
  "3720_0_1": "Linha_3720_060453_G_2022-06-23-22-06.pdf",
  "3720_0_2": "Linha_3720_150013_R_2022-06-23-22-12.pdf",
  "3721_0_1": "Linha_3721_060453_G_2022-06-24-00-16.pdf",
  "3721_0_2": "Linha_3721_150051_R_2022-06-24-00-16.pdf",
  "4101_0_1": "Linha_4101_090001_G_2022-06-29-15-28.pdf",
  "4101_0_2": "Linha_4101_090044_R_2022-06-29-15-28.pdf",
  "4102_0_1": "Linha_4102_090052_G_2022-06-29-16-50.pdf",
  "4102_0_2": "Linha_4102_090109_R_2022-06-29-16-50.pdf",
  "4103_0_1": "Linha_4103_090052_G_2022-06-29-15-27.pdf",
  "4103_0_2": "Linha_4103_090111_R_2022-06-29-15-27.pdf",
  "4201_0_1": "Linha_4201_100002_G_2022-06-29-15-47.pdf",
  "4201_0_2": "Linha_4201_100021_R_2022-06-29-15-47.pdf",
  "4202_0_1": "Linha_4202_100001_G_2022-06-29-15-31.pdf",
  "4202_0_2": "Linha_4202_100089_R_2022-06-29-15-31.pdf",
  "4203_0_1": "Linha_4203_100001_G_2022-06-29-15-30.pdf",
  "4203_0_2": "Linha_4203_100137_R_2022-06-29-15-30.pdf",
  "4204_0_1": "Linha_4204_100145_G_2022-06-29-15-29.pdf",
  "4204_0_2": "Linha_4204_100157_R_2022-06-29-15-29.pdf",
  "4205_0_1": "Linha_4205_010075_G_2022-06-29-15-28.pdf",
  "4205_0_2": "Linha_4205_100157_R_2022-06-29-15-28.pdf",
  "4206_0_1": "Linha_4206_100061_G_2022-06-29-15-29.pdf",
  "4206_0_2": "Linha_4206_100137_R_2022-06-29-15-29.pdf",
  "4207_0_1": "Linha_4207_100153_G_2022-06-29-15-48.pdf",
  "4207_0_2": "Linha_4207_100157_R_2022-06-29-15-48.pdf",
  "4208_0_1": "Linha_4208_100013_G_2022-06-29-15-31.pdf",
  "4208_0_2": "Linha_4208_100197_R_2022-06-29-15-31.pdf",
  "4210_0_1": "Linha_4210_100201_G_2022-06-29-15-32.pdf",
  "4210_0_2": "Linha_4210_100214_R_2022-06-29-15-32.pdf",
  "4212_0_1": "Linha_4212_100214_G_2022-06-29-15-31.pdf",
  "4212_0_2": "Linha_4212_100281_R_2022-06-29-15-31.pdf",
  "4301_0_1": "Linha_4301_130001_G_2022-06-29-15-50.pdf",
  "4301_0_2": "Linha_4301_130007_R_2022-06-29-15-50.pdf",
  "4302_0_2": "Linha_4302_130007_R_2022-06-29-15-49.pdf",
  "4302_0_1": "Linha_4302_130009_G_2022-06-29-15-49.pdf",
  "4304_0_1": "Linha_4304_130065_G_2022-06-29-15-49.pdf",
  "4304_0_2": "Linha_4304_130093_R_2022-06-29-15-49.pdf",
  "4305_0_2": "Linha_4305_130007_R_2022-06-29-15-48.pdf",
  "4305_0_1": "Linha_4305_130100_G_2022-06-29-15-48.pdf",
  "4306_0_2": "Linha_4306_130007_R_2022-06-29-15-52.pdf",
  "4306_0_1": "Linha_4306_130116_G_2022-06-29-15-52.pdf",
  "4307_0_2": "Linha_4307_130007_R_2022-06-29-15-49.pdf",
  "4307_0_1": "Linha_4307_130139_G_2022-06-29-15-49.pdf",
  "4308_0_1": "Linha_4308_130007_G_2022-06-29-15-53.pdf",
  "4308_0_2": "Linha_4308_130230_R_2022-06-29-15-53.pdf",
  "4310_0_2": "Linha_4310_130162_R_2022-06-29-15-50.pdf",
  "4310_0_1": "Linha_4310_130708_G_2022-06-29-15-50.pdf",
  "4311_0_2": "Linha_4311_130262_R_2022-06-29-15-50.pdf",
  "4311_0_1": "Linha_4311_130262_G_2022-06-29-15-50.pdf",
  "4312_0_1": "Linha_4312_130256_G_2022-06-29-15-51.pdf",
  "4312_0_2": "Linha_4312_160009_R_2022-06-29-15-51.pdf",
  "4313_0_2": "Linha_4313_130093_R_2022-06-29-15-52.pdf",
  "4313_0_1": "Linha_4313_130116_G_2022-06-29-15-52.pdf",
  "4321_0_2": "Linha_4321_130084_R_2022-06-29-15-53.pdf",
  "4321_0_1": "Linha_4321_130345_G_2022-06-29-15-53.pdf",
  "4322_0_1": "Linha_4322_130378_G_2022-06-29-15-49.pdf",
  "4322_0_2": "Linha_4322_130435_R_2022-06-29-15-49.pdf",
  "4403_0_1": "Linha_4403_160073_G_2022-06-29-16-22.pdf",
  "4403_0_2": "Linha_4403_160101_R_2022-06-29-16-22.pdf",
  "4407_0_1": "Linha_4407_160051_G_2022-06-29-16-08.pdf",
  "4408_0_2": "Linha_4408_160161_R_2022-06-29-15-55.pdf",
  "4412_0_2": "Linha_4412_160161_R_2022-06-29-15-54.pdf",
  "4412_0_1": "Linha_4412_160305_G_2022-06-29-15-54.pdf",
  "4413_0_2": "Linha_4413_160161_R_2022-06-29-15-54.pdf",
  "4414_0_2": "Linha_4414_160067_R_2022-06-29-16-11.pdf",
  "4414_0_1": "Linha_4414_160328_G_2022-06-29-16-11.pdf",
  "4415_0_2": "Linha_4415_160067_R_2022-06-29-16-11.pdf",
  "4415_0_1": "Linha_4415_160328_G_2022-06-29-16-11.pdf",
  "4416_0_2": "Linha_4416_160067_R_2022-06-29-15-57.pdf",
  "4416_0_1": "Linha_4416_160371_G_2022-06-29-15-57.pdf",
  "4417_0_2": "Linha_4417_160067_R_2022-06-29-16-21.pdf",
  "4417_0_1": "Linha_4417_160371_G_2022-06-29-16-21.pdf",
  "4418_0_2": "Linha_4418_160139_R_2022-06-29-16-21.pdf",
  "4418_0_1": "Linha_4418_160297_G_2022-06-29-16-21.pdf",
  "4419_0_2": "Linha_4419_160347_R_2022-06-29-16-06.pdf",
  "4419_0_1": "Linha_4419_160379_G_2022-06-29-16-06.pdf",
  "4420_0_2": "Linha_4420_160067_R_2022-06-29-15-57.pdf",
  "4420_0_1": "Linha_4420_160297_G_2022-06-29-15-57.pdf",
  "4424_0_1": "Linha_4424_160266_G_2022-06-29-16-02.pdf",
  "4424_0_2": "Linha_4424_160477_R_2022-06-29-16-02.pdf",
  "4425_0_1": "Linha_4425_160268_G_2022-06-29-16-01.pdf",
  "4425_0_2": "Linha_4425_160502_R_2022-06-29-16-01.pdf",
  "4426_0_2": "Linha_4426_160237_R_2022-06-29-16-01.pdf",
  "4426_0_1": "Linha_4426_160266_G_2022-06-29-16-01.pdf",
  "4431_0_1": "Linha_4431_160068_G_2022-06-29-15-58.pdf",
  "4431_0_2": "Linha_4431_160529_R_2022-06-29-15-58.pdf",
  "4432_0_1": "Linha_4432_160067_G_2022-06-29-15-54.pdf",
  "4432_0_2": "Linha_4432_160596_R_2022-06-29-15-54.pdf",
  "4433_0_2": "Linha_4433_160451_R_2022-06-29-16-07.pdf",
  "4433_0_1": "Linha_4433_160609_G_2022-06-29-16-07.pdf",
  "4434_0_1": "Linha_4434_160020_G_2022-06-29-16-00.pdf",
  "4434_0_2": "Linha_4434_160629_R_2022-06-29-16-00.pdf",
  "4435_0_1": "Linha_4435_160640_G_2022-06-29-16-15.pdf",
  "4435_0_2": "Linha_4435_160684_R_2022-06-29-16-15.pdf",
  "4436_0_1": "Linha_4436_160161_G_2022-06-29-15-56.pdf",
  "4436_0_2": "Linha_4436_160209_R_2022-06-29-15-56.pdf",
  "4438_0_2": "Linha_4438_160347_R_2022-06-29-16-06.pdf",
  "4438_0_1": "Linha_4438_160690_G_2022-06-29-16-06.pdf",
  "4440_0_2": "Linha_4440_160347_R_2022-06-29-16-05.pdf",
  "4440_0_1": "Linha_4440_160690_G_2022-06-29-16-05.pdf",
  "4441_0_1": "Linha_4441_160347_G_2022-06-29-16-07.pdf",
  "4441_0_2": "Linha_4441_160399_R_2022-06-29-16-07.pdf",
  "4442_0_2": "Linha_4442_160285_R_2022-06-29-16-12.pdf",
  "4442_0_1": "Linha_4442_160701_G_2022-06-29-16-12.pdf",
  "4451_0_2": "Linha_4451_160067_R_2022-06-29-16-13.pdf",
  "4451_0_1": "Linha_4451_160502_G_2022-06-29-16-13.pdf",
  "4452_0_2": "Linha_4452_160067_R_2022-06-29-16-12.pdf",
  "4452_0_1": "Linha_4452_160731_G_2022-06-29-16-12.pdf",
  "4453_0_2": "Linha_4453_160067_R_2022-06-29-16-12.pdf",
  "4453_0_1": "Linha_4453_160731_G_2022-06-29-16-12.pdf",
  "4460_0_1": "Linha_4460_160747_G_2022-06-29-16-15.pdf",
  "4460_0_2": "Linha_4460_160747_R_2022-06-29-16-15.pdf",
  "4470_0_1": "Linha_4470_160791_G_2022-06-29-17-38.pdf",
  "4470_0_2": "Linha_4470_160801_R_2022-06-29-17-38.pdf",
  "4472_0_2": "Linha_4472_160067_R_2022-06-29-16-08.pdf",
  "4472_0_1": "Linha_4472_160802_G_2022-06-29-16-08.pdf",
  "4474_0_1": "Linha_4474_160819_G_2022-06-29-16-09.pdf",
  "4474_0_2": "Linha_4474_160831_R_2022-06-29-16-09.pdf",
  "4475_0_2": "Linha_4475_160268_R_2022-06-29-16-21.pdf",
  "4475_0_1": "Linha_4475_160833_G_2022-06-29-16-21.pdf",
  "4501_0_1": "Linha_4501_010079_G_2022-06-29-16-27.pdf",
  "4501_0_2": "Linha_4501_100157_R_2022-06-29-16-27.pdf",
  "4502_0_1": "Linha_4502_010079_G_2022-06-29-16-31.pdf",
  "4502_0_2": "Linha_4502_010175_R_2022-06-29-16-31.pdf",
  "4503_0_2": "Linha_4503_100044_R_2022-06-29-16-25.pdf",
  "4503_0_1": "Linha_4503_100164_G_2022-06-29-16-25.pdf",
  "4504_0_2": "Linha_4504_010133_R_2022-06-29-16-26.pdf",
  "4504_0_1": "Linha_4504_100137_G_2022-06-29-16-26.pdf",
  "4510_0_1": "Linha_4510_010136_G_2022-06-29-16-29.pdf",
  "4510_0_2": "Linha_4510_100013_R_2022-06-29-16-29.pdf",
  "4511_0_1": "Linha_4511_010136_G_2022-06-29-16-42.pdf",
  "4511_0_2": "Linha_4511_100013_R_2022-06-29-16-42.pdf",
  "4512_0_1": "Linha_4512_010136_G_2022-06-29-16-28.pdf",
  "4512_0_2": "Linha_4512_160068_R_2022-06-29-16-28.pdf",
  "4513_0_1": "Linha_4513_010136_G_2022-06-29-16-29.pdf",
  "4513_0_2": "Linha_4513_130228_R_2022-06-29-16-29.pdf",
  "4514_0_2": "Linha_4514_100013_R_2022-06-29-16-30.pdf",
  "4514_0_1": "Linha_4514_100201_G_2022-06-29-16-30.pdf",
  "4515_0_1": "Linha_4515_100013_G_2022-06-29-16-31.pdf",
  "4515_0_2": "Linha_4515_100368_R_2022-06-29-16-31.pdf",
  "4516_0_1": "Linha_4516_100013_G_2022-06-29-16-32.pdf",
  "4516_0_2": "Linha_4516_130505_R_2022-06-29-16-32.pdf",
  "4517_0_2": "Linha_4517_160068_R_2022-06-29-16-30.pdf",
  "4520_0_2": "Linha_4520_100281_R_2022-06-29-16-32.pdf",
  "4520_0_1": "Linha_4520_100392_G_2022-06-29-16-32.pdf",
  "4521_0_1": "Linha_4521_100392_G_2022-06-29-16-34.pdf",
  "4521_0_2": "Linha_4521_130346_R_2022-06-29-16-34.pdf",
  "4522_0_1": "Linha_4522_100392_G_2022-06-29-16-34.pdf",
  "4522_0_2": "Linha_4522_130256_R_2022-06-29-16-34.pdf",
  "4523_0_2": "Linha_4523_130228_R_2022-06-29-16-29.pdf",
  "4524_0_2": "Linha_4524_100368_R_2022-06-29-16-35.pdf",
  "4524_0_1": "Linha_4524_130007_G_2022-06-29-16-35.pdf",
  "4530_0_1": "Linha_4530_090137_G_2022-06-29-16-35.pdf",
  "4530_0_2": "Linha_4530_130378_R_2022-06-29-16-35.pdf",
  "4531_0_1": "Linha_4531_090057_G_2022-06-29-16-25.pdf",
  "4531_0_2": "Linha_4531_130007_R_2022-06-29-16-25.pdf",
  "4532_0_1": "Linha_4532_090058_G_2022-06-29-16-25.pdf",
  "4532_0_2": "Linha_4532_090145_R_2022-06-29-16-25.pdf",
  "4540_0_1": "Linha_4540_130708_G_2022-06-29-16-38.pdf",
  "4540_0_2": "Linha_4540_160067_R_2022-06-29-16-38.pdf",
  "4541_0_1": "Linha_4541_130107_G_2022-06-29-16-36.pdf",
  "4541_0_2": "Linha_4541_160347_R_2022-06-29-16-36.pdf",
  "4542_0_1": "Linha_4542_130107_G_2022-06-29-16-41.pdf",
  "4542_0_2": "Linha_4542_160067_R_2022-06-29-16-41.pdf",
  "4543_0_1": "Linha_4543_130107_G_2022-06-29-16-42.pdf",
  "4543_0_2": "Linha_4543_160067_R_2022-06-29-16-42.pdf",
  "4544_0_1": "Linha_4544_130246_G_2022-06-29-16-37.pdf",
  "4544_0_2": "Linha_4544_160067_R_2022-06-29-16-37.pdf",
  "4545_0_2": "Linha_4545_160286_R_2022-06-29-16-36.pdf",
  "4545_0_1": "Linha_4545_160640_G_2022-06-29-16-36.pdf",
  "4546_0_2": "Linha_4546_160067_R_2022-06-29-16-41.pdf",
  "4546_0_1": "Linha_4546_160640_G_2022-06-29-16-41.pdf",
  "4547_0_1": "Linha_4547_130116_G_2022-06-29-16-38.pdf",
  "4547_0_2": "Linha_4547_160067_R_2022-06-29-16-38.pdf",
  "4548_0_1": "Linha_4548_130174_G_2022-06-29-16-36.pdf",
  "4548_0_2": "Linha_4548_160067_R_2022-06-29-16-36.pdf",
  "4549_0_1": "Linha_4549_130007_G_2022-06-29-16-40.pdf",
  "4549_0_2": "Linha_4549_160067_R_2022-06-29-16-40.pdf",
  "4550_0_1": "Linha_4550_130065_G_2022-06-29-16-40.pdf",
  "4550_0_2": "Linha_4550_160747_R_2022-06-29-16-40.pdf",
  "4551_0_1": "Linha_4551_130040_G_2022-06-29-16-27.pdf",
  "4551_0_2": "Linha_4551_160101_R_2022-06-29-16-27.pdf",
  "4560_0_1": "Linha_4560_130442_G_2022-06-29-16-24.pdf",
  "4561_0_2": "Linha_4561_130442_R_2022-06-29-16-24.pdf",
  "4561_0_1": "Linha_4561_160927_G_2022-06-29-16-24.pdf",
  "4562_0_1": "Linha_4562_160067_G_2022-06-29-16-39.pdf",
  "4562_0_2": "Linha_4562_160747_R_2022-06-29-16-39.pdf",
  "4600_0_1": "Linha_4600_010136_G_2022-06-29-16-48.pdf",
  "4600_0_2": "Linha_4600_040013_R_2022-06-29-16-48.pdf",
  "4601_0_1": "Linha_4601_040013_G_2022-06-29-16-51.pdf",
  "4601_0_2": "Linha_4601_100013_R_2022-06-29-16-51.pdf",
  "4602_0_2": "Linha_4602_040013_R_2022-06-29-16-47.pdf",
  "4602_0_1": "Linha_4602_090211_G_2022-06-29-16-47.pdf",
  "4603_0_1": "Linha_4603_040013_G_2022-06-29-16-50.pdf",
  "4603_0_2": "Linha_4603_090181_R_2022-06-29-16-50.pdf",
  "4604_0_1": "Linha_4604_040027_G_2022-06-29-16-49.pdf",
  "4604_0_2": "Linha_4604_090095_R_2022-06-29-16-49.pdf",
  "4605_0_1": "Linha_4605_040018_G_2022-06-29-16-48.pdf",
  "4605_0_2": "Linha_4605_090283_R_2022-06-29-16-48.pdf",
  "4610_0_2": "Linha_4610_040027_R_2022-06-29-16-46.pdf",
  "4610_0_1": "Linha_4610_130714_G_2022-06-29-16-46.pdf",
  "4611_0_1": "Linha_4611_040096_G_2022-06-29-16-49.pdf",
  "4611_0_2": "Linha_4611_090165_R_2022-06-29-16-49.pdf",
  "4612_0_2": "Linha_4612_130065_R_2022-06-29-16-47.pdf",
  "4612_0_1": "Linha_4612_130715_G_2022-06-29-16-47.pdf",
  "4620_0_1": "Linha_4620_090057_G_2022-06-29-16-44.pdf",
  "4620_0_2": "Linha_4620_140021_R_2022-06-29-16-44.pdf",
  "4621_0_1": "Linha_4621_090057_G_2022-06-29-16-44.pdf",
  "4621_0_2": "Linha_4621_140073_R_2022-06-29-16-44.pdf",
  "4630_0_1": "Linha_4630_140089_G_2022-06-29-16-46.pdf",
  "4630_0_2": "Linha_4630_160067_R_2022-06-29-16-46.pdf",
  "4631_0_1": "Linha_4631_140139_G_2022-06-29-16-45.pdf",
  "4631_0_2": "Linha_4631_160067_R_2022-06-29-16-45.pdf",
  "4640_0_2": "Linha_4640_160936_R_2022-06-29-16-44.pdf",
  "4640_0_1": "Linha_4640_160942_G_2022-06-29-16-44.pdf",
  "4641_0_1": "Linha_4641_150013_G_2022-06-29-16-46.pdf",
  "4641_0_2": "Linha_4641_160067_R_2022-06-29-16-46.pdf",
  "4642_0_1": "Linha_4642_150052_G_2022-06-29-16-43.pdf",
  "4642_0_2": "Linha_4642_160029_R_2022-06-29-16-43.pdf",
  "4643_0_1": "Linha_4643_100089_G_2022-06-29-16-44.pdf",
  "4643_0_2": "Linha_4643_150052_R_2022-06-29-16-44.pdf",
  "4701_0_1": "Linha_4701_060001_G_2022-06-29-18-17.pdf",
  "4701_0_2": "Linha_4701_090260_R_2022-06-29-18-17.pdf",
  "4702_0_2": "Linha_4702_010083_R_2022-06-29-17-40.pdf",
  "4702_0_1": "Linha_4702_060001_G_2022-06-29-17-40.pdf",
  "4703_0_1": "Linha_4703_060001_G_2022-06-29-18-19.pdf",
  "4703_0_2": "Linha_4703_100013_R_2022-06-29-18-19.pdf",
  "4704_0_2": "Linha_4704_060001_R_2022-06-29-18-19.pdf",
  "4704_0_1": "Linha_4704_100172_G_2022-06-29-18-19.pdf",
  "4705_0_2": "Linha_4705_010099_R_2022-06-29-18-20.pdf",
  "4705_0_1": "Linha_4705_060001_G_2022-06-29-18-20.pdf",
  "4706_0_1": "Linha_4706_010059_G_2022-06-29-18-20.pdf",
  "4706_0_2": "Linha_4706_060001_R_2022-06-29-18-20.pdf",
  "4707_0_1": "Linha_4707_060001_G_2022-06-29-17-41.pdf",
  "4707_0_2": "Linha_4707_100027_R_2022-06-29-17-41.pdf",
  "4710_0_1": "Linha_4710_060009_G_2022-06-29-17-43.pdf",
  "4710_0_2": "Linha_4710_130007_R_2022-06-29-17-43.pdf",
  "4711_0_2": "Linha_4711_130454_R_2022-06-29-17-43.pdf",
  "4715_0_1": "Linha_4715_060009_G_2022-06-29-17-43.pdf",
  "4715_0_2": "Linha_4715_160068_R_2022-06-29-17-43.pdf",
  "4720_0_1": "Linha_4720_060011_G_2022-06-29-17-55.pdf",
  "4720_0_2": "Linha_4720_160068_R_2022-06-29-17-55.pdf",
  "4725_0_1": "Linha_4725_060013_G_2022-06-29-17-41.pdf",
  "4725_0_2": "Linha_4725_160068_R_2022-06-29-17-41.pdf",
  "4901_0_2": "Linha_4901_160067_R_2022-06-29-16-59.pdf",
  "4901_0_1": "Linha_4901_190002_G_2022-06-29-16-59.pdf",
  "4902_0_1": "Linha_4902_100282_G_2022-06-29-16-59.pdf",
  "4902_0_2": "Linha_4902_190002_R_2022-06-29-16-59.pdf",
  "4905_0_1": "Linha_4905_100435_G_2022-06-29-16-58.pdf",
  "4905_0_2": "Linha_4905_190008_R_2022-06-29-16-58.pdf",
  "4906_0_1": "Linha_4906_160067_G_2022-06-29-16-59.pdf",
  "4906_0_2": "Linha_4906_190008_R_2022-06-29-16-59.pdf"
};
const linhas_e_tarifario = {
  "3001": "PrÃ³xima",
  "3002": "Longa",
  "3003": "Longa",
  "3004": "Longa",
  "3005": "PrÃ³xima",
  "3006": "PrÃ³xima",
  "3007": "Longa",
  "3008": "Longa",
  "3009": "Longa",
  "3010": "Longa",
  "3011": "Longa",
  "3012": "Longa",
  "3013": "Longa",
  "3014": "Longa",
  "3015": "Longa",
  "3016": "Longa",
  "3017": "Longa",
  "3018": "Longa",
  "3019": "Longa",
  "3020": "PrÃ³xima",
  "3021": "Longa",
  "3022": "Longa",
  "3023": "Longa",
  "3024": "Longa",
  "3025": "Longa",
  "3026": "PrÃ³xima",
  "3027": "Longa",
  "3028": "PrÃ³xima",
  "3029": "PrÃ³xima",
  "3030": "Longa",
  "3031": "Longa",
  "3032": "Longa",
  "3033": "PrÃ³xima",
  "3034": "Longa",
  "3035": "Longa",
  "3036": "Longa",
  "3037": "PrÃ³xima",
  "3101": "PrÃ³xima",
  "3102": "Longa",
  "3103": "Longa",
  "3104": "Longa",
  "3105": "Longa",
  "3106": "PrÃ³xima",
  "3107": "PrÃ³xima",
  "3108": "PrÃ³xima",
  "3109": "Longa",
  "3110": "Longa",
  "3111": "Longa",
  "3112": "PrÃ³xima",
  "3113": "PrÃ³xima",
  "3114": "Longa",
  "3115": "Longa",
  "3116": "Longa",
  "3117": "Longa",
  "3118": "Longa",
  "3119": "Longa",
  "3120": "Longa",
  "3121": "PrÃ³xima",
  "3122": "Longa",
  "3201": "Longa",
  "3202": "Longa",
  "3203": "Longa",
  "3204": "Longa",
  "3205": "Longa",
  "3206": "Longa",
  "3207": "Longa",
  "3208": "Longa",
  "3209": "Longa",
  "3210": "Longa",
  "3211": "Longa",
  "3212": "Longa",
  "3213": "Longa",
  "3214": "Longa",
  "3215": "Longa",
  "3216": "Longa",
  "3217": "Longa",
  "3218": "PrÃ³xima",
  "3219": "PrÃ³xima",
  "3220": "PrÃ³xima",
  "3221": "Longa",
  "3222": "PrÃ³xima",
  "3501": "Longa",
  "3502": "Longa",
  "3503": "Longa",
  "3504": "Longa",
  "3505": "Longa",
  "3506": "Longa",
  "3507": "Longa",
  "3508": "Longa",
  "3509": "Longa",
  "3510": "Longa",
  "3511": "Longa",
  "3512": "Longa",
  "3513": "Longa",
  "3514": "Longa",
  "3515": "Longa",
  "3516": "Longa",
  "3517": "Longa",
  "3518": "Longa",
  "3519": "Longa",
  "3520": "Longa",
  "3521": "Longa",
  "3522": "Longa",
  "3523": "Longa",
  "3524": "Longa",
  "3525": "Longa",
  "3526": "Longa",
  "3527": "Longa",
  "3528": "Longa",
  "3535": "Longa",
  "3536": "Longa",
  "3540": "Longa",
  "3541": "Longa",
  "3542": "Longa",
  "3543": "Longa",
  "3544": "Longa",
  "3545": "Longa",
  "3546": "Longa",
  "3547": "Longa",
  "3548": "Longa",
  "3549": "Longa",
  "3601": "Longa",
  "3605": "Longa",
  "3610": "Longa",
  "3615": "Longa",
  "3620": "Longa",
  "3625": "Longa",
  "3626": "Longa",
  "3630": "Longa",
  "3635": "Longa",
  "3640": "Longa",
  "3641": "Longa",
  "3650": "Mar",
  "3701": "Longa",
  "3702": "Longa",
  "3703": "Longa",
  "3704": "Longa",
  "3705": "Longa",
  "3706": "Longa",
  "3707": "Longa",
  "3708": "Longa",
  "3709": "Longa",
  "3710": "Longa",
  "3711": "Longa",
  "3715": "Longa",
  "3716": "Longa",
  "3717": "Longa",
  "3720": "RÃ¡pida",
  "3721": "RÃ¡pida",
  "4001": "PrÃ³xima",
  "4002": "Longa",
  "4101": "Longa",
  "4102": "Longa",
  "4103": "PrÃ³xima",
  "4104": "PrÃ³xima",
  "4201": "Longa",
  "4202": "Longa",
  "4203": "PrÃ³xima",
  "4204": "PrÃ³xima",
  "4205": "Longa",
  "4206": "PrÃ³xima",
  "4207": "PrÃ³xima",
  "4208": "Longa",
  "4210": "Longa",
  "4211": "PrÃ³xima",
  "4212": "Longa",
  "4301": "PrÃ³xima",
  "4302": "Longa",
  "4303": "PrÃ³xima",
  "4304": "Longa",
  "4305": "Longa",
  "4306": "Longa",
  "4307": "Longa",
  "4308": "Longa",
  "4310": "Longa",
  "4311": "Longa",
  "4312": "Longa",
  "4313": "Longa",
  "4320": "PrÃ³xima",
  "4321": "Longa",
  "4322": "Longa",
  "4401": "Longa",
  "4402": "Longa",
  "4403": "PrÃ³xima",
  "4404": "PrÃ³xima",
  "4405": "Longa",
  "4406": "PrÃ³xima",
  "4407": "Longa",
  "4408": "Longa",
  "4409": "Longa",
  "4410": "Longa",
  "4411": "Longa",
  "4412": "Longa",
  "4413": "Longa",
  "4414": "Longa",
  "4415": "Longa",
  "4416": "Longa",
  "4417": "Longa",
  "4418": "Longa",
  "4419": "Longa",
  "4420": "PrÃ³xima",
  "4421": "Longa",
  "4422": "Longa",
  "4423": "PrÃ³xima",
  "4424": "Longa",
  "4425": "Longa",
  "4426": "PrÃ³xima",
  "4427": "PrÃ³xima",
  "4428": "Longa",
  "4429": "Longa",
  "4430": "PrÃ³xima",
  "4431": "PrÃ³xima",
  "4432": "Longa",
  "4433": "Longa",
  "4434": "PrÃ³xima",
  "4435": "Longa",
  "4436": "PrÃ³xima",
  "4437": "Longa",
  "4438": "Longa",
  "4439": "Longa",
  "4440": "Longa",
  "4441": "Longa",
  "4442": "Longa",
  "4443": "Longa",
  "4451": "Longa",
  "4452": "Longa",
  "4453": "Longa",
  "4460": "Longa",
  "4470": "Longa",
  "4471": "Longa",
  "4472": "Longa",
  "4474": "Longa",
  "4475": "Longa",
  "4476": "Longa",
  "4501": "Longa",
  "4502": "Longa",
  "4503": "Longa",
  "4504": "Longa",
  "4510": "Longa",
  "4511": "Longa",
  "4512": "Longa",
  "4513": "Longa",
  "4514": "Longa",
  "4515": "Longa",
  "4516": "Longa",
  "4517": "Longa",
  "4520": "Longa",
  "4521": "Longa",
  "4522": "Longa",
  "4523": "Longa",
  "4524": "Longa",
  "4530": "Longa",
  "4531": "Longa",
  "4532": "Longa",
  "4540": "Longa",
  "4541": "Longa",
  "4542": "Longa",
  "4543": "Longa",
  "4544": "Longa",
  "4545": "Longa",
  "4546": "Longa",
  "4547": "Longa",
  "4548": "Longa",
  "4549": "Longa",
  "4550": "Longa",
  "4551": "Longa",
  "4560": "Longa",
  "4561": "Longa",
  "4562": "Longa",
  "4600": "Longa",
  "4601": "Longa",
  "4602": "Longa",
  "4603": "Longa",
  "4604": "Longa",
  "4605": "Longa",
  "4610": "Longa",
  "4611": "Longa",
  "4612": "Longa",
  "4620": "Longa",
  "4621": "Longa",
  "4630": "Longa",
  "4631": "Longa",
  "4640": "Longa",
  "4641": "Longa",
  "4642": "Longa",
  "4643": "Mar",
  "4701": "RÃ¡pida",
  "4702": "RÃ¡pida",
  "4703": "RÃ¡pida",
  "4704": "RÃ¡pida",
  "4705": "RÃ¡pida",
  "4706": "RÃ¡pida",
  "4707": "RÃ¡pida",
  "4710": "RÃ¡pida",
  "4711": "RÃ¡pida",
  "4715": "RÃ¡pida",
  "4720": "RÃ¡pida",
  "4725": "RÃ¡pida",
  "4901": "Longa",
  "4902": "Longa",
  "4905": "Inter-regional",
  "4906": "Inter-regional"
};
// document.addEventListener("DOMContentLoaded", function () {
//   initSvgMap();
// });
const YELLOW_CITIES = ["sintra", "amadora", "oeiras"];
const ORANGE_CITIES = ["almada", "sexial", "sesimbra"];
const GREEN_CITIES = ["marfa", "loures", "xira", "odivelas"];
const BLUE_CITIES = ["montijo", "moita", "alcochete", "palmela", "montijo2", "setubal"];
const GREY_CITIES = ["cascais", "lisboa", "barreiro"];
const YELLOW_DATA = {
  title: "Carris Metropolitana operada por ViaÃ§Ã£o Alvorada",
  duration: "",
  footer: [{
    text: "Idade media da frota no inicio",
    text2: "Menos de 7 meses"
  }, {
    text: "Remuneracao anual de referencia",
    text2: "56,7 MilhÃµes"
  }],
  schemes: [{
    label: "Oferta reforÃ§ada",
    num: "83"
  }, {
    label: "Total de linhas",
    num: "192"
  }, {
    label: "Novas",
    num: "41"
  }]
};
const ORANGE_DATA = {
  title: "Carris Metropolitana operada por Transportes Sul do Tejo",
  duration: "",
  footer: [{
    text: "Idade media da frota no inicio",
    text2: "Cerca de 11 meses"
  }, {
    text: "Remuneracao anual de referencia",
    text2: "37 MilhÃµes"
  }],
  schemes: [{
    label: "Oferta reforÃ§ada",
    num: "56"
  }, {
    label: "Total de linhas",
    num: "149"
  }, {
    label: "Novas",
    num: "50"
  }]
};
const GREEN_DATA = {
  title: "Carris Metropolitana operada por RodoviÃ¡ria de Lisboa",
  duration: "",
  footer: [{
    text: "Idade media da frota no inicio",
    text2: "Menos de 8 meses"
  }, {
    text: "Remuneracao anual de referencia",
    text2: "52,4 MilhÃµes"
  }],
  schemes: [{
    label: "Oferta reforÃ§ada",
    num: "134"
  }, {
    label: "Total de linhas",
    num: "327"
  }, {
    label: "Novas",
    num: "41"
  }]
};
const BLUE_DATA = {
  title: "Carris Metropolitana operada por Alsa Todi",
  duration: "",
  footer: [{
    text: "Idade media da frota no inicio",
    text2: "Novos"
  }, {
    text: "Remuneracao anual de referencia",
    text2: "26,4 MilhÃµes"
  }],
  schemes: [{
    label: "Oferta reforÃ§ada",
    num: "74"
  }, {
    label: "Total de linhas",
    num: "154"
  }, {
    label: "Novas",
    num: "30"
  }]
};
const GREY_DATA = {
  title: "",
  duration: "",
  footer: [],
  schemes: []
};
const DATA = [YELLOW_CITIES.map(c => Object.assign({
  id: c
}, YELLOW_DATA)), ORANGE_CITIES.map(c => Object.assign({
  id: c
}, ORANGE_DATA)), GREEN_CITIES.map(c => Object.assign({
  id: c
}, GREEN_DATA)), BLUE_CITIES.map(c => Object.assign({
  id: c
}, BLUE_DATA)), GREY_CITIES.map(c => Object.assign({
  id: c
}, GREY_DATA))].flat(1);

function initSvgMap() {
  let root = document.documentElement;
  const map = document.querySelector("#svg-map");
  if (!map) return;
  const segments = map.querySelectorAll(".segment");
  const MISSING_TEXT = "A OperaÃ§Ã£o da rede municipal no Barreiro, em Cascais e em Lisboa continua a ser realizada por operadores internos.";

  function getPos(i) {
    switch (i) {
      case 0:
        return "left";

      case 1:
        return "middle";

      case 2:
        return "right";

      case 3:
        return "bottom";

      default:
        break;
    }
  }

  function getTemplate(data) {
    const withOutContent = GREY_CITIES.includes(data.id);
    const tooltip = document.createElement("div");
    tooltip.classList.add("tooltip");
    const header = document.createElement("div");
    header.classList.add("tooltip__header");
    const headerButton = document.createElement("button");
    headerButton.classList.add("tooltip__close-btn");
    headerButton.addEventListener('click', function () {
      window.tippy.hideAll();
    });
    const main = document.createElement("div");
    main.classList.add("tooltip__main-block");
    main.appendChild(header);

    if (withOutContent) {
      header.appendChild(headerButton);
      tooltip.classList.add("js-missing-data");
      const missingData = document.createElement("div");
      missingData.classList.add("tooltip__missing-text");
      missingData.innerText = MISSING_TEXT;
      main.appendChild(missingData);
      tooltip.appendChild(main);
    } else {
      const headerTitle = document.createElement("span");
      headerTitle.classList.add("tooltip__title");
      headerTitle.innerText = data.title;
      header.appendChild(headerTitle);
      header.appendChild(headerButton);
      const scheme = document.createElement("div");
      scheme.classList.add("tooltip__scheme");

      for (let i = 0; i < data.schemes.length; i++) {
        const pos = getPos(i);
        const item = document.createElement("div");
        const boldText = document.createElement("b");
        const spanText = document.createElement("span");
        boldText.innerText = data.schemes[i].num;
        spanText.innerText = data.schemes[i].label;
        item.classList.add("scheme-item", "scheme-item--" + pos);
        item.appendChild(boldText);
        item.appendChild(spanText);
        scheme.appendChild(item);
      }

      main.appendChild(scheme);
      const footer = document.createElement("div");
      footer.classList.add("tooltip__bottom-block");

      for (let i = 0; i < data.footer.length; i++) {
        const row = document.createElement("div");
        row.classList.add("tooltip__bottom-block__row");
        const span = document.createElement("span");
        const b = document.createElement("b");
        span.innerText = data.footer[i].text;
        b.innerText = data.footer[i].text2;
        row.appendChild(span);
        row.appendChild(b);
        footer.appendChild(row);
      }

      tooltip.appendChild(main);
      tooltip.appendChild(footer);
    }

    return tooltip;
  }

  function initTippy(triggerTarget, target, tooltipData) {
    const template = getTemplate(tooltipData);
    tippy(target, {
      allowHTML: true,
      animation: "scale",
      arrow: false,
      trigger: "click",
      triggerTarget: triggerTarget,
      content: template,
      boundary: window,
      interactive: true,
      appendTo: document.body
    });
  }

  function getDataById(id) {
    return DATA.find(c => c.id == id);
  }

  function getSegmentSettings(nodes) {
    const activeColor = nodes.rect[0].getAttribute("fill");
    return {
      tagColor: "#fff",
      segmentColor: "#eee",
      textColor: "#111",
      activeColor: activeColor
    };
  }

  function setActivePathStyles(nodes, settings) {
    nodes.elem.style.fill = settings.activeColor;
    nodes.rect[0].style.fill = settings.tagColor;
    nodes.textPath.style.fill = settings.textColor;
  }

  function removeActivePathStyles(nodes, settings) {
    nodes.elem.style.fill = settings.segmentColor;
    nodes.rect[0].style.fill = settings.activeColor;
    nodes.textPath.style.fill = settings.tagColor;
  }

  function removeActiveStylesFromAllPaths() {
    const activeElements = map.querySelectorAll(".segment.js-active");
    activeElements.forEach(function (elem) {
      const attr = elem.getAttribute("data-target");
      const rect = document.querySelectorAll("rect[data-target=" + attr + "]");
      const textPath = document.querySelector("path[data-target=" + attr + "]:not(.segment)");
      const nodes = {
        rect,
        textPath,
        elem
      };
      const settings = getSegmentSettings(nodes);
      elem.classList.remove("js-active");
      removeActivePathStyles(nodes, settings);
    });
  }

  function addListeners(nodes, settings, tooltipData) {
    nodes.elem.addEventListener("mouseover", function () {
      setActivePathStyles(nodes, settings);
    });
    nodes.elem.addEventListener("mouseout", function () {
      if (nodes.elem.classList.contains("js-active")) return;
      removeActivePathStyles(nodes, settings);
    });
    nodes.elem.addEventListener("click", function () {
      removeActiveStylesFromAllPaths();
      nodes.elem.classList.add("js-active");
      root.style.setProperty("--tt-active-color", settings.activeColor);
      setActivePathStyles(nodes, settings);
      window.scrollIntoView(nodes.rect[0], {
        behavior: "smooth",
        scrollMode: "if-needed",
        block: "nearest"
      });
    });
    document.addEventListener("click", function (event) {
      if (!event.target.hasAttribute("data-target")) {
        removeActiveStylesFromAllPaths();
      }
    });
  }

  segments.forEach(function (elem) {
    const attr = elem.getAttribute("data-target");
    const rect = document.querySelectorAll("rect[data-target=" + attr + "]");
    const textPath = document.querySelector("path[data-target=" + attr + "]:not(.segment)");
    const nodes = {
      rect,
      textPath,
      elem
    };
    const settings = getSegmentSettings(nodes);
    const tooltipData = getDataById(attr);
    addListeners(nodes, settings, tooltipData);
    initTippy(elem, rect[0], tooltipData);
  });
  const ww = window.outerWidth;

  if (ww < 767) {
    const container = document.querySelector('#map-container');
    const innerContainer = document.querySelector('#map-inner-container');
    const innerConteinerWidth = window.getComputedStyle(innerContainer).width;
    const containerOffset = +innerConteinerWidth.replace('px', '');
    const offset = containerOffset / 2 - ww / 2;
    container.scroll({
      top: 0,
      left: offset,
      behavior: 'auto'
    });
  }
}
const plano_oferta = {
  "1": ["1", "1", "1", "1", "1", "1", "1", "1", "1"],
  "2": ["1", "0", "1", "1", "0", "1", "1", "0", "1"],
  "3": ["1", "1", "0", "1", "1", "0", "1", "1", "0"],
  "4": ["1", "0", "0", "1", "0", "0", "1", "0", "0"],
  "5": ["0", "0", "1", "0", "0", "1", "0", "0", "1"],
  "7": ["0", "1", "0", "0", "1", "0", "0", "1", "0"],
  "8": ["0", "1", "1", "0", "1", "1", "0", "1", "1"],
  "11": ["1f", "1", "0", "1f", "1", "0", "1f", "1", "0"],
  "15": ["1", "0", "0", "0", "0", "0", "0", "0", "0"],
  "20": ["1e", "0", "0", "0", "0", "0", "0", "0", "0"],
  "21": ["1d", "0", "0", "0", "0", "0", "0", "0", "0"],
  "25": ["1b", "0", "0", "0", "0", "0", "0", "0", "0"],
  "28": ["1c", "0", "0", "0", "0", "0", "0", "0", "0"],
  "30": ["1f", "0", "0", "0", "0", "0", "0", "0", "0"],
  "36": ["1", "0", "0", "1", "0", "0", "0", "0", "0"],
  "38": ["0", "1", "0", "0", "1", "0", "0", "0", "0"],
  "39": ["0", "1", "1", "0", "1", "1", "0", "0", "0"],
  "40": ["1", "1", "1", "1", "1", "1", "0", "0", "0"],
  "41": ["0", "0", "0", "1", "0", "0", "1", "0", "0"],
  "50": ["0", "0", "0", "1", "0", "0", "0", "0", "0"],
  "51": ["0", "0", "0", "0", "0", "0", "1", "0", "0"],
  "52": ["0", "0", "0", "0", "0", "0", "0", "1", "0"],
  "53": ["0", "0", "0", "0", "0", "0", "0", "0", "1"],
  "54": ["0", "0", "0", "0", "0", "0", "0", "1", "1"],
  "56": ["0", "0", "0", "0", "0", "0", "1", "1", "1"],
  "60": ["0", "1", "0", "1", "1", "0", "1", "1", "0"],
  "62": ["1", "0", "0", "0", "1", "0", "0", "1", "0"],
  "63": ["1", "1", "0", "0", "1", "0", "0", "1", "0"],
  "66": ["1", "0", "0", "0", "0", "0", "1a", "0", "0"],
  "69": ["0", "0", "1h", "0", "0", "1h", "0", "0", "1h"],
  "77": ["1", "0", "0", "1", "1", "0", "1", "1", "0"],
  "87": ["0", "0", "1i", "0", "0", "1i", "0", "0", "1i"],
  "97": ["0", "0", "1g", "0", "0", "1g", "0", "0", "1g"],
  "100": ["1", "1", "0", "0", "1", "0", "0", "1", "0"],
  "102": ["0", "1", "0", "0", "1", "0", "1", "1", "0"],
  "109": ["0", "0", "1", "1", "0", "1", "1", "0", "1"],
  "111": ["0", "1", "1", "0", "1", "1", "1", "1", "1"],
  "112": ["1", "1", "1", "1", "1", "1", "0", "1", "1"],
  "113": ["0", "0", "1", "1", "1", "1", "1", "1", "1"],
  "115": ["1", "1", "1", "0", "1", "1", "0", "1", "1"],
  "118": ["1j", "1j", "1j", "1j", "1j", "1j", "1", "1", "1"],
  "119": ["1", "1", "1", "1", "1", "1", "1", "1", "1"],
  "120": ["0", "1j", "1j", "0", "1j", "1j", "1l", "1", "1"],
  "121": ["1k", "1j", "1j", "1k", "1j", "1j", "1", "1", "1"]
};
const plano_oferta_legenda = {
  "a": "SÃ³ Ã s segundas-feiras",
  "b": "SÃ³ Ã s segundas e terÃ§as-feiras",
  "c": "SÃ³ Ã s segundas, terÃ§as, quintas e sextas-feiras",
  "d": "SÃ³ Ã s quartas-feiras",
  "e": "SÃ³ Ã s quartas, quintas e sextas-feiras",
  "f": "SÃ³ Ã s sextas-feiras",
  "g": "Primeiro domingo de cada mÃªs",
  "h": "Segundo domingo de cada mÃªs",
  "i": "Exceto primeiro domingo de cada mÃªs",
  "j": "Apenas entre 10 de junho e 15 de setembro",
  "k": "Apenas entre 1 e 15 de setembro",
  "l": "Apenas durante o mÃªs de agosto"
};
////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Cria lista agentes 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////
function listaAgentes() {
  // Cria seletor de municipios ------------------------------------------------------------------------------
  var text_s = '<select id="municipioAgentes" class="nomePesquisa" style="background-color: #ffdd00">';
  text_s += '<option value="escolha">MunicÃ­pio</option>';

  for (let municipio in diretorio_agentes) {
    text_s += `<option value="${municipio}">${municipio}</option>`;
  }

  text_s += '</select></div>';
  text_s += '<div id="agentes" style="display:none"></div>';
  document.querySelector("#divAgentes").innerHTML = text_s; // Cria lista de pontos ------------------------------------------------------------------------------

  document.querySelector("#municipioAgentes").onchange = function () {
    offsetId('redeAgentes');
    var municipio = document.querySelector("#municipioAgentes").value;
    var text = '';

    if (municipio in diretorio_pontos_navegante && municipio in diretorio_espacos_navegante) {
      text += '<div class="titulo">EspaÃ§os e pontos naveganteÂ®</div>';
    } else if (municipio in diretorio_pontos_navegante && !(municipio in diretorio_espacos_navegante)) {
      text += '<div class="titulo">Pontos naveganteÂ®</div>';
    } else if (!(municipio in diretorio_pontos_navegante) && municipio in diretorio_espacos_navegante) {
      text += '<div class="titulo">EspaÃ§os naveganteÂ®</div>';
    } // Pontos Navegante ****************************************************


    if (municipio in diretorio_pontos_navegante) {
      text += '<table id="tablePontoNavegante">'; // text += `<tr>`;
      // text += `    <th>Nome</th>`;
      // text += `    <th>Morada</th>`;
      // text += `    <th>HorÃ¡rio</th>`;
      // text += `</tr>`;

      text += `<tr>`;
      text += `    <th>Ponto naveganteÂ®</th>`;
      text += `    <td>${diretorio_pontos_navegante[municipio][1]}<br>${diretorio_pontos_navegante[municipio][2]}</td>`;
      text += `    <td>${diretorio_pontos_navegante[municipio][3].replace(', ', '<br>')}</td>`;
      text += `</tr>`;
    } // EspaÃ§o Navegante


    if (municipio in diretorio_espacos_navegante) {
      diretorio_espacos_navegante[municipio].forEach(ponto => {
        text += `<tr>`;
        text += `    <th>EspaÃ§o naveganteÂ®</th>`;
        text += `    <td class='nome'>${ponto[0]}</td>`;
        text += `    <td class='morada'>${ponto[1]}</td>`;
        text += `</tr>`;
      });
    }

    text += `</table>`; // Rede de agentes

    text += '<br>';
    text += '<div class="titulo">Agentes agrupados por freguesia</div>';
    text += '<table id="tableAgentes">'; // text += `<tr class='tituloTableAgentes'>`;
    // text += `    <th class='freguesia'>Freguesia</th>`;
    // text += `    <th>Nome</th>`;
    // text += `    <th>Morada</th>`;
    // text += `</tr>`;

    freguesia_anterior = diretorio_agentes[municipio][3];
    diretorio_agentes[municipio].forEach(info => {
      let nome = info[0];
      let morada = info[1];
      let codigo_postal = info[2];
      let freguesia = info[3];
      text += `<tr class='agente'>`;

      if (freguesia != freguesia_anterior) {
        text += `    <th class="freguesia borda" style="background:#ffdd00">${freguesia}</th>`;
        text += `    <td class="nome borda">${nome}</td>`;
        text += `    <td class="morada borda">${morada}, ${codigo_postal}, ${freguesia}</td>`;
      } else {
        text += `    <th class="freguesia"></th>`;
        text += `    <td class="nome">${nome}</td>`;
        text += `    <td class="morada">${morada}, ${codigo_postal}, ${freguesia}</td>`;
      }

      freguesia_anterior = freguesia;
      text += `</tr>`;
    });
    text += `</table>`; // Adicionar ao elemento

    document.getElementById("agentes").innerHTML = text;
    document.getElementById("agentes").style.display = 'block';
  };
} ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Listener 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////


document.addEventListener('DOMContentLoaded', () => {
  listaAgentes();
});