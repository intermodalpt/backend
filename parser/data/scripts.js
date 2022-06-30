function converteDeMunicipioLinhaAntiga() {
    let a = "<div>Selecione o MunicÃ­pio</div>";
    a += "<div>", a += '<select id="seletorMunicipios" class="nomePesquisa">', a += '<option value="escolha">escolha</option>';
    for (let o in diretorio) a += `<option value="${o}">${o}</option>`;
    a += "</select>", a += "</div>", document.getElementById("divSeletorMunicipios").innerHTML = a, document.querySelector("#seletorMunicipios").onchange = function () {
        document.querySelector("input").value = "", document.getElementById("seletorOperadores").selectedIndex = 0, document.querySelector("#linhasConvertidasOperador").innerHTML = "", document.getElementById("listaNovasLinhas").innerHTML = "", document.getElementById("seletorNovasLinhasMunicipio").selectedIndex = 0, document.getElementById("carreiras").innerHTML = "", document.querySelector("#divOperadoresLinhas").innerHTML = "";
        let a = document.querySelector("#seletorMunicipios").value,
            o = '<div>Linha atual</div><select id="seletorLinhasMunicipio" class="numLinhaPesquisa"><option value="">nÂº</option>';
        for (const s in diretorio[a]) "Nova" != s && (o += `<option value="${s}">${s}</option>`);
        o += "</select>", document.getElementById("divMunicipiosLinhas").innerHTML = o
    }, document.querySelector("#divMunicipiosLinhas").onchange = function () {
        let a, o = document.querySelector("#seletorLinhasMunicipio").value,
            s = document.querySelector("#seletorMunicipios").value;
        a = 1 == diretorio[s][o].length ? "Nova linha correspondente:<br>" : "Novas linhas correspondentes:<br>";
        for (let i in diretorio[s][o]) {
            a += "<div class='novaLinha'>", a += `    <div class="novaLinhaNum">${diretorio[s][o][i][0]}</div>`, a += `    <div class="novaLinhaNome">${diretorio[s][o][i][1]}</div>`, a += "</div>"
        }
        document.getElementById("linhasConvertidasMunicipio").innerHTML = a
    }
}

function converteDeOperadorLinhaAntiga() {
    let a = "<div>Selecione o Operador</div>";
    a += "<div>", a += '<select id="seletorOperadores" class="nomePesquisa">', a += '<option value="escolha">escolha</option>';
    for (let o in diretorio_operadores) "Nova" != o && (a += `<option value="${o}">${o}</option>`);
    a += "</select>", a += "</div>", document.getElementById("divSeletorOperadores").innerHTML = a, document.querySelector("#seletorOperadores").onchange = function () {
        document.querySelector("input").value = "", document.getElementById("seletorMunicipios").selectedIndex = 0, document.querySelector("#linhasConvertidasMunicipio").innerHTML = "", document.getElementById("listaNovasLinhas").innerHTML = "", document.querySelector("#divMunicipiosLinhas").innerHTML = "", document.getElementById("seletorNovasLinhasMunicipio").selectedIndex = 0, document.getElementById("carreiras").innerHTML = "", document.getElementById("linhasConvertidasOperador").innerHTML = "", document.getElementById("divOperadoresLinhas").innerHTML = "";
        let a = document.querySelector("#seletorOperadores").value,
            o = '<div>Linha atual</div><select id="seletorLinhasOperador" class="numLinhaPesquisa"><option value="">escolha</option>';
        for (const s in diretorio_operadores[a]) "Nova" != s && (o += `<option value="${s}">${s}</option>`);
        o += "</select>", document.getElementById("divOperadoresLinhas").innerHTML = o
    }, document.querySelector("#divOperadoresLinhas").onchange = function () {
        let a, o = document.querySelector("#seletorLinhasOperador").value,
            s = document.querySelector("#seletorOperadores").value;
        a = 1 == diretorio_operadores[s][o].length ? "<b>Nova linha correspondente</b><br>" : "<b>Novas linhas correspondentes</b><br>";
        for (let i in diretorio_operadores[s][o]) {
            a += "<div class='novaLinha'>", a += `    <div class="novaLinhaNum">${diretorio_operadores[s][o][i][0]}</div>`, a += `    <div class="novaLinhaNome">${diretorio_operadores[s][o][i][1]}</div>`, a += "</div>"
        }
        document.getElementById("linhasConvertidasOperador").innerHTML = a
    }
}

function listaNovasLinhasDeMunicipio() {
    let a = '<div class="opcoesPesquisa"><div>';
    a += "<div>Selecione o MunicÃ­pio</div>", a += '<select id="seletorNovasLinhasMunicipio" class="nomePesquisa">', a += '<option value="escolha">escolha</option>', linhas = [];
    for (let o in diretorio) {
        for (let a in diretorio[o]) diretorio[o][a];
        a += `<option value="${o}">${o}</option>`
    }
    a += "</select></div></div>", a += '<div id="listaNovasLinhas"></div>', document.querySelector("#novasLinhasMunicipio").innerHTML = a, document.querySelector("#seletorNovasLinhasMunicipio").onchange = function () {
        document.getElementById("seletorOperadores").selectedIndex = 0, document.querySelector("#linhasConvertidasOperador").innerHTML = "", document.getElementById("listaNovasLinhas").innerHTML = "", document.querySelector("#divOperadoresLinhas").innerHTML = "", document.getElementById("seletorMunicipios").selectedIndex = 0, document.querySelector("#linhasConvertidasMunicipio").innerHTML = "", document.getElementById("listaNovasLinhas").innerHTML = "", document.querySelector("#divMunicipiosLinhas").innerHTML = "";
        let a = document.querySelector("#seletorNovasLinhasMunicipio").value, o = "";
        for (let s in diretorio[a].Nova) {
            o += "<div class='novaLinha'>", o += `    <div class="novaLinhaNum">${diretorio[a].Nova[s][0]}</div>`, o += `    <div class="novaLinhaNome">${diretorio[a].Nova[s][1]}</div>`, o += "</div>"
        }
        diretorio[a].Nova.length > 0 && (document.getElementById("listaNovasLinhas").innerHTML = o)
    }
}

function pesquisaMunicipio() {
    var a = document.getElementById("meuMunicipio").value;
    meuMunicipio = a.charAt(0).toUpperCase() + a.slice(1).toLowerCase(), meuMunicipio in diretorio ? document.getElementById("municipio").innerHTML = meuMunicipio + " Ã© um municipio do diretorio" : document.getElementById("municipio").innerHTML = " "
}

function pesquisaNovaCarreira() {
    var a = document.getElementById("minhaCarreira").value, o = {};
    for (let s in diretorio) if (a in diretorio[s]) for (let i in diretorio[s][a]) c = diretorio[s][a][i][0], d = diretorio[s][a][i][1], o[c] = d;
    var s = Object.keys(o).length;
    if (0 == s) i = ""; else {
        var i;
        i = 1 == s ? "<p>Nova linha correspondente:</p>" : "<p>Novas linhas correspondentes:</p>";
        for (let a in o) i += "<div class='novaLinha'>", i += `   <div class="novaLinhaNum">${a}</div>`, i += `   <div class="novaLinhaNome">${o[a]}</div>`, i += "</div>"
    }
    document.getElementById("carreiras").innerHTML = i
}

jQuery(document).ready((function () {
    !function (a) {
        a(".js-accordion-btn").on("click", (function (o) {
            var s = a(this).closest("li").find(".js-accordion-content");
            a(this).closest(".js-accordion-container").find(".js-accordion-content").not(s).slideUp(), a(this).hasClass("active") ? (a(this).removeClass("active"), a(this).closest("li").removeClass("active")) : (a(this).closest(".js-accordion-container").find(".js-accordion-btn.active").removeClass("active"), a(this).addClass("active"), a(".js-accordion-container").find("li").removeClass("active"), a(this).closest("li").addClass("active")), s.stop(!1, !0).slideToggle(), o.preventDefault()
        }))
    }(jQuery)
})), document.addEventListener("DOMContentLoaded", (() => {
    converteDeMunicipioLinhaAntiga(), converteDeOperadorLinhaAntiga(), listaNovasLinhasDeMunicipio()
}));
const diretorio = {
    Alcochete: {
        410: [["4511", "Alcochete (Freeport) - Montijo (Terminal RodoviÃ¡rio), via Samouco"], ["4600", "Alcochete (Freeport) - Barreiro (Terminal)"]],
        "412 (Adaptado)": [["4501", "Alcochete - Montijo (Terminal Fluvial)"]],
        413: [["4510", "Alcochete (Freeport) - Montijo (Terminal RodoviÃ¡rio)"], ["4512", "Alcochete (Freeport) - SetÃºbal (ITS), via Alto Estanqueiro"], ["4513", "Alcochete (Freeport) - Pinhal Novo"]],
        414: [["4514", "Canha - Montijo (Terminal RodoviÃ¡rio), via PegÃµes"]],
        415: [["4504", "Montijo (Terminal Fluvial) - Passil"]],
        416: [["4514", "Canha - Montijo (Terminal RodoviÃ¡rio), via PegÃµes"], ["4515", "Montijo (Terminal RodoviÃ¡rio) - PegÃµes"]],
        419: [["4502", "Alcochete - Passil"]],
        426: [["4516", "Montijo (Terminal RodoviÃ¡rio) - Rio Frio"]],
        431: [["4703", "Lisboa (Oriente) - Montijo (Terminal RodoviÃ¡rio), via Alcochete e Samouco"]],
        432: [["4702", "Lisboa (Oriente) - Valbom"], ["4704", "Atalaia - Lisboa (Oriente)"]],
        435: [["4705", "Lisboa (Oriente) - Samouco"]],
        453: [["4706", "SÃ£o Francisco - Lisboa (Oriente)"]],
        Nova: [["4001", "Alcochete | Circular"], ["4002", "SÃ£o Francisco | Circular"], ["4503", "Atalaia - Jardia"]]
    },
    "Alenquer (CIM Oeste)": {
        16: [["2928", "Casais da Granja - Vila Franca de Xira (EstaÃ§Ã£o)"]],
        18: [["2925", "Cadafais - Vila Franca de Xira (Terminal), via Bairro da Mata"], ["2926", "Cadafais - Vila Franca de Xira (Terminal), via Cachoeiras"]],
        19: [["2927", "Cadafais - Vila Franca de Xira (Terminal), via Loja Nova"]]
    },
    Almada: {
        101: [["3001", "Almada (Cristo Rei) - Cacilhas (Terminal)"]],
        102: [["3003", "Almada Forum - Cacilhas (Terminal)"]],
        106: [["3007", "Bairro Fundo Fomento - Cacilhas (Terminal)"]],
        110: [["3513", "Cacilhas (Terminal) - Santa Marta do Pinhal"]],
        114: [["3508", "Cacilhas (Terminal) - Paio Pires (Centro)"]],
        116: [["3521", "Cruz de Pau - Fonta da Telha"], ["3523", "Fonte da Telha - Paio Pires (Quinta FlamÃ¢ncia), via Seixal (Terminal Fluvial) e Foros de Amora (EstaÃ§Ã£o)"]],
        117: [["3014", "Cacilhas (terminal) - Raposeira"]],
        120: [["3515", "Caparica (Pilotos) -  Corroios"]],
        121: [["3526", "Laranjeiro - Pinheirinho"]],
        "123 (Adaptado)": [["3013", "Cacilhas (Terminal) - Monte da Caparica"]],
        "124 (Adaptado)": [["3022", "Costa da Caparica (Terminal) - Hospital Garcia de Orta"]],
        "126 (Adaptado)": [["3507", "Cacilhas (Terminal) - Marisol"], ["3524", "Hospital Garcia de Orta - Marisol"]],
        "127 (Adaptado)": [["3012", "Cacilhas (Terminal) - Fonte da Telha"]],
        "129 (Adaptado)": [["3030", "Fonte da Telha - Monte da Caparica (FCT)"]],
        "130 (Adaptado)": [["3030", "Fonte da Telha - Monte da Caparica (FCT)"]],
        135: [["3011", "Cacilhas (Terminal) - Costa da Caparica"]],
        139: [["3519", "Costa da Caparica (Terminal) - Corroios (EstaÃ§Ã£o)"]],
        143: [["3518", "Corroios (EstaÃ§Ã£o) - Vale de Figueira"]],
        "145 (Adaptado)": [["3010", "Cacilhas (Terminal) - Charneca da Caparica"]],
        146: [["3008", "BanÃ¡tica - Quintinha"]],
        "146 (Adaptado)": [["3032", "Monte da Caparica (FCT) - Quinta do Texugo"], ["3034", "Porto BrandÃ£o (Terminal) - Quinta do Texugo"]],
        149: [["3512", "Cacilhas (Terminal) - Quinta Princesa"]],
        151: [["3704", "Charneca da Caparica - Lisboa (M. Pombal)"]],
        153: [["3710", "Costa da Caparica (Terminal) - Lisboa (Sete Rios)"]],
        155: [["3709", "Costa da Caparica (Terminal) - Lisboa (M. Pombal)"]],
        158: [["3711", "Monte da Caparica (FCT) - Lisboa (Sete Rios)"]],
        159: [["3707", "Charneca da Caparica - Lisboa (Sete Rios), via Sobreda"], ["3716", "Lisboa (Sete Rios) - Marisol"]],
        "159 (adaptada)": [["3501", "Almada Forum - Marisol, via Sobreda"]],
        160: [["3703", "Almada (Parque Urbano) - Lisboa (Sete Rios)"]],
        161: [["3710", "Costa da Caparica (Terminal) - Lisboa (Sete Rios)"]],
        162: [["3717", "Lisboa (Sete Rios) - Quinta do Brasileiro"]],
        163: [["3520", "Costa da Caparica (Terminal) - Quinta do Brasileiro"]],
        167: [["3023", "Costa da Caparica (terminal) - Laranjeiro"]],
        169: [["3715", "Lisboa (M. Pombal) - Santa Marta do Pinhal"]],
        171: [["3015", "Charneca da Caparica - Cova do Vapor"]],
        172: [["3522", "Fonte da Telha - Paio Pires (Centro)"]],
        "174 (Adaptado)": [["3024", "Costa da Caparica (Terminal) - Pragal (EstaÃ§Ã£o)"]],
        "175 (adaptada)": [["3501", "Almada Forum - Marisol, via Sobreda"]],
        176: [["3702", "Almada (Parque Urbano) - Lisboa (C. UniversitÃ¡ria)"]],
        "179(adaptada)": [["3004", "Almada Forum - Marisol"]],
        180: [["3018", "Charneca da Caparica - Sobreda"]],
        "181 (adaptada)": [["3525", "Hospital Garcia de Orta - Miratejo"]],
        182: [["3026", "Cova da Piedade - Hospital Garcia de Orta"]],
        "190 (adaptada)": [["3705", "Charneca da Caparica - Lisboa (Sete Rios)"]],
        191: [["3514", "Cacilhas (Terminal) - Vale de MilhaÃ§os"]],
        192: [["3511", "Cacilhas (Terminal) - Pinheirinho"]],
        196: [["3510", "Cacilhas (Terminal) - Pilotos"]],
        197: [["3504", "Bairro Fundo Fomento - Quintinha"]],
        198: [["3502", "Almada Forum - Paio Pires (Centro)"], ["3527", "Monte da Caparica (FCT) - Paio Pires (Bairro Cucena)"], ["3528", "Monte da Caparica (FCT) - Paio Pires (Centro)"]],
        199: [["3509", "Cacilhas (Terminal) - Paio Pires (Centro), via Seixal (Terminal Fluvial) e Amora"]],
        "1C": [["3516", "Charneca da Caparica - Corroios (EstaÃ§Ã£o)"]],
        "1P": [["3027", "Hospital Garcia de Orta - Sobreda"]],
        203: [["3536", "Cacilhas (Terminal) - Sesimbra (Terminal)"]],
        207: [["3721", "Lisboa (Sete Rios) - Sesimbra (Terminal)"]],
        233: [["2850", "Costa da Caparica - PÃ³voa de Santo AdriÃ£o (Parque Urbano)"]],
        252: [["3720", "Lisboa (Sete Rios) - Quinta do Conde"]],
        254: [["3535", "Cacilhas (Terminal) - Quinta do Conde"]],
        "2C (Adaptado)": [["3506", "Cacilhas (Terminal) - Corroios (EstaÃ§Ã£o), via Miratejo"]],
        355: [["2651", "Bucelas - Costa da Caparica (Terminal)"]],
        "3C": [["3517", "Chegadinho - Corroios (EstaÃ§Ã£o)"]],
        470: [["2650", "Cidade Nova - Costa da Caparica"]],
        561: [["4725", "Lisboa (Sete Rios) - SetÃºbal (ITS)"]],
        583: [["3610", "Cacilhas (Terminal) - SetÃºbal (ITS), via A2"]],
        701: [["2652", "Costa da Caparica (Terminal) - Forte da Casa"]],
        783: [["3605", "Cacilhas (Terminal) - SetÃºbal (ITS), via AzeitÃ£o"]],
        Flexibus: [["3005", "Flexibus Almada | Circular"]],
        Nova: [["3002", "Almada (Parque Urbano) - Pragal (EstaÃ§Ã£o)"], ["3006", "Aroeira | Circular"], ["3009", "Cacilhas (terminal - Trafaria (Terminal)"], ["3016", "Charneca da Caparica - Lazarim"], ["3017", "Charneca da Caparica - Pragal (EstaÃ§Ã£o)"], ["3019", "Charneca da Caparica - Trafaria (Terminal)"], ["3020", "Charneca da Caparica | Circular"], ["3021", "Costa da Caparica - Monte da Caparica (FCT)"], ["3025", "Costa da Caparica (Terminal) - Pragal (EstaÃ§Ã£o), via IC20"], ["3028", "Lazarim | Circular"], ["3029", "Marco CabaÃ§o | Circular"], ["3031", "Monte da Caparica - Quintinha"], ["3033", "Monte da Caparica | Circular"], ["3035", "Pragal (EstaÃ§Ã£o) - Quinta do Texugo"], ["3036", "Pragal (EstaÃ§Ã£o) - Vale Flores"], ["3037", "Quintinha | Circular"], ["3503", "Almada Forum - Vale de MilhaÃ§os"], ["3505", "Cacilhas (Terminal) - Corroios (EstaÃ§Ã£o)"], ["3601", "Barreiro - Cova da Piedade (Metro)"], ["3701", "Almada (Centro Sul) - AlgÃ©s (Terminal)"], ["3706", "Charneca da Caparica - Lisboa (Sete Rios), via FeijÃ³"], ["3708", "Costa da Caparica (Terminal) - Lisboa (C. SodrÃ©)"]]
    },
    Amadora: {
        1: [["1504", "AlgÃ©s (Terminal) - Bairro Zambujal, via Linda-a-Velha"]],
        10: [["1503", "AlgÃ©s (Terminal) - Bairro Zambujal"], ["1505", "AlgÃ©s (Terminal) - IKEA Alfragide"]],
        101: [["1717", "Lisboa (C. Militar) - Tercena, via Amadora Este (Metro)"]],
        103: [["1512", "Amadora (Hospital) - Montelavar"]],
        104: [["1508", "Almargem do Bispo - Amadora Este (Metro)"]],
        105: [["1518", "Monte AbraÃ£o - Reboleira (Metro)"]],
        106: [["1601", "Amadora Este (Metro) - Carcavelos (Praia)"], ["1602", "Carcavelos (Praia) - Queluz"]],
        107: [["1716", "Idanha - Lisboa (M. Pombal)"]],
        108: [["1507", "Caxias - Reboleira (Metro)"]],
        113: [["1714", "Amadora (EstaÃ§Ã£o Sul) - BelÃ©m (EstaÃ§Ã£o)"]],
        114: [["1502", "AlgÃ©s (Terminal) - Amadora (EstaÃ§Ã£o Sul), via Linda-a-Velha"]],
        "118 (Adaptado)": [["1004", "Amadora (EstaÃ§Ã£o Norte) - Moinhos da Funcheira | noturna"], ["1006", "Amadora (EstaÃ§Ã£o Norte) - UBBO | noturna"]],
        128: [["1701", "Alto Brandoa - Benfica | madrugada"], ["1702", "Alto Brandoa - Lisboa (C. Militar) | madrugada"], ["1706", "Lisboa (C. Militar) - UBBO"]],
        132: [["1515", "Amadora Este (Metro) - Casal de Cambra"], ["1720", "Casal de Cambra - Lisboa (C. Militar), via Amadora"]],
        133: [["1603", "Amadora (EstaÃ§Ã£o Norte) - CaneÃ§as"]],
        134: [["1517", "Casal de Cambra - Reboleira (Metro)"]],
        "136 (Adaptado)": [["1004", "Amadora (EstaÃ§Ã£o Norte) - Moinhos da Funcheira | noturna"]],
        "137 (Adaptado)": [["1005", "Amadora (EstaÃ§Ã£o Norte) - UBBO"]],
        142: [["1705", "Benfica (GrÃ£o Vasco) - UBBO, via Amadora Este (Metro) | noturna e madrugada"], ["1707", "Lisboa (C. Militar) - UBBO, via Falagueira"]],
        143: [["1007", "Amadora (EstaÃ§Ã£o Norte) | Circular madrugada"], ["1703", "Amadora (EstaÃ§Ã£o Norte) - Pontinha (Metro)"]],
        144: [["1718", "CacÃ©m (Bairro Grajal) - BelÃ©m (EstaÃ§Ã£o)"]],
        149: [["1715", "BelÃ©m (EstaÃ§Ã£o) - Mira Sintra (Mercado)"]],
        "149/Nova": [["1511", "Amadora (Hospital) - Monte AbraÃ£o (EstaÃ§Ã£o)"]],
        "154 (Adaptado)": [["1514", "Amadora (Hospital) | Circular, via Brandoa"]],
        "155 (Adaptado)": [["1514", "Amadora (Hospital) | Circular, via Brandoa"]],
        157: [["1519", "Queluz (PalÃ¡cio) - Serra da Silveira"]],
        162: [["1713", "AlgÃ©s (Terminal) - Amadora Este (Metro)"]],
        163: [["1721", "Lisboa (C. Militar) - MassamÃ¡ (Casal do Olival), via Amadora Este (Metro)"]],
        165: [["1708", "UBBO | Circular"]],
        "168 (Adaptado)": [["1011", "Brandoa (Largo) - Reboleira (Metro)"]],
        185: [["1704", "Amadora (Hospital) - Lisboa (M. Pombal)"]],
        "186 (Adaptado)": [["1510", "Amadora (Hospital) - Damaia (Praceta Liberdade)"]],
        20: [["1712", "AlgÃ©s (Terminal) - Amadora (EstaÃ§Ã£o Sul)"]],
        206: [["2601", "Loures (C.C. Continente) - Reboleira (Metro)"]],
        210: [["1709", "CaneÃ§as - Lisboa (C. Militar)"], ["1710", "CaneÃ§as - Pontinha (Metro)"]],
        215: [["2605", "CacÃ©m (EstaÃ§Ã£o) - Loures (Lg Marcos RomÃ£o Reis JÃºnior)"], ["2606", "CacÃ©m (EstaÃ§Ã£o) - CaneÃ§as (Casa da Cultura)"]],
        224: [["1711", "CaneÃ§as (Esc. SecundÃ¡ria) - Pontinha (Metro)"]],
        227: [["2701", "Pontinha (Metro) - Vale Grande"]],
        231: [["1709", "CaneÃ§as - Lisboa (C. Militar)"]],
        25: [["1513", "Amadora (Hospital) | Circular"]],
        "26 (Adaptado)": [["1003", "Amadora (EstaÃ§Ã£o Norte) - Amadora Este (Metro)"]],
        931: [["2702", "Lisboa (C. Grande) - Pontinha (Metro)"]],
        Nova: [["1001", "Alfragide (Estrada do Seminario) - Reboleira (EstaÃ§Ã£o)"], ["1002", "Alfragide (Igreja da Divina MisericÃ³rdia) - Amadora"], ["1008", "Amadora Este | Circular"], ["1009", "Amadora Hospital | Circular"], ["1010", "Brandoa (Esc. SecundÃ¡ria) - Casal da Mira"], ["1012", "Brandoa | Circular"], ["1013", "Casas do Lago - Damaia (Escola Doutor Azevedo Neves)"], ["1014", "Casas do Lago - Vila ChÃ£"], ["1015", "Reboleira | Circular"], ["1501", "Alfragide - Reboleira (EstaÃ§Ã£o) | Circular"], ["1506", "Amadora Hospital | Circular, via Alfragide"], ["1509", "Amadora (Hospital) - Casal de Cambra (C. SaÃºde)"], ["1516", "Casal de Cambra - Monte AbraÃ£o (EstaÃ§Ã£o)"], ["1719", "Casal de Cambra - Lisboa (C. Militar)"], ["2610", "Odivelas (Metro) - UBBO"], ["2611", "UBBO - Ramada"]]
    },
    "Arruda dos Vinhos (CIM Oeste)": {
        16: [["2928", "Casais da Granja - Vila Franca de Xira (EstaÃ§Ã£o)"]],
        215: [["2901", "Rolia - Venda do Pinheiro (Terminal)"]],
        58: [["2920", "A-do-MourÃ£o - Alverca (EstaÃ§Ã£o)"]],
        59: [["2921", "Alhandra - Arruda dos Vinhos"], ["2922", "Arruda dos Vinhos - Bulhaco"]]
    },
    Barreiro: {
        "1N": [["3620", "Coina (EstaÃ§Ã£o) - Quinta do Conde"]],
        245: [["3650", "Moita - Sesimbra (Terminal)"]],
        "2N": [["3626", "Coina (EstaÃ§Ã£o) - Vila Fresca de AzeitÃ£o"]],
        302: [["4620", "Moita - Paio Pires"]],
        305: [["4610", "Bairro dos Marinheiros - Barreiro (Terminal)"]],
        "307 (Adaptado)": [["4604", "Barreiro (Terminal) - Moita (Escola Fragata do Tejo)"]],
        "311 (Adaptado)": [["4611", "Penalva - Moita (Esc. SecundÃ¡ria)"]],
        317: [["4602", "Alhos Vedros (EstaÃ§Ã£o) - Barreiro (Terminal)"]],
        330: [["4605", "Lavradio - Pinhal do Forno"]],
        410: [["4600", "Alcochete (Freeport) - Barreiro (Terminal)"], ["4601", "Barreiro (Terminal) - Montijo (Terminal RodoviÃ¡rio)"], ["4603", "Barreiro (Terminal) - ChÃ£o Duro"]],
        754: [["4631", "Fogueteiro (EstaÃ§Ã£o) - SetÃºbal (ITS)"]],
        755: [["4630", "Corroios (EstaÃ§Ã£o) - SetÃºbal (ITS)"]],
        783: [["3605", "Cacilhas (Terminal) - SetÃºbal (ITS), via AzeitÃ£o"]],
        Nova: [["3601", "Barreiro - Cova da Piedade (Metro)"], ["3615", "Barreiro - Seixal"], ["3625", "Barreiro - Sesimbra (Terminal)"], ["4612", "Bairro dos Marinheiros - Palmela (Terminal)"], ["4621", "Moita - Seixal (Terminal Fluvial)"]]
    },
    Cascais: {
        106: [["1601", "Amadora Este (Metro) - Carcavelos (Praia)"], ["1602", "Carcavelos (Praia) - Queluz"]],
        119: [["1617", "PaÃ§o de Arcos (EstaÃ§Ã£o) - TalaÃ­de (Igreja)"]],
        125: [["1610", "PaÃ§o de Arcos (EstaÃ§Ã£o) - TalaÃ­de (Campo de Futebol)"], ["1616", "PaÃ§o de Arcos (EstaÃ§Ã£o) - Taguspark"]],
        184: [["1611", "PaÃ§o de Arcos (EstaÃ§Ã£o) - TalaÃ­de (Campo de Futebol), via Vila Fria"]],
        403: [["1624", "Cascais (Terminal) - Portela de Sintra (EstaÃ§Ã£o), via AzÃ³ia e AlmoÃ§ageme"]],
        417: [["1623", "Cascais (Terminal) - Portela de Sintra (EstaÃ§Ã£o)"], ["1626", "CascaiShopping - Portela de Sintra (EstaÃ§Ã£o)"]],
        418: [["1621", "Bairro da Cruz Vermelha - Portela de Sintra (EstaÃ§Ã£o)"], ["1626", "CascaiShopping - Portela de Sintra (EstaÃ§Ã£o)"], ["1629", "Estoril (EstaÃ§Ã£o) - Portela de Sintra (EstaÃ§Ã£o)"], ["1630", "Estoril (EstaÃ§Ã£o) - Portela de Sintra (EstaÃ§Ã£o), via Monte Estoril e Amoreira"]],
        455: [["1625", "Cascais (Terminal) - Rio de Mouro (EstaÃ§Ã£o)"]],
        456: [["1627", "CascaiShopping - Rio de Mouro (EstaÃ§Ã£o)"], ["1628", "Estoril (EstaÃ§Ã£o) -  Rio de Mouro (EstaÃ§Ã£o)"], ["1631", "Estoril (EstaÃ§Ã£o) - Rio de Mouro (EstaÃ§Ã£o) | Direta"]],
        463: [["1613", "CacÃ©m (EstaÃ§Ã£o) - Oeiras (EstaÃ§Ã£o), via Trajouce"], ["1618", "AbÃ³boda (Auto BarÃ£o) - CacÃ©m (EstaÃ§Ã£o)"], ["1622", "CacÃ©m (EstaÃ§Ã£o) - Carcavelos (EstaÃ§Ã£o), via Trajouce"]],
        467: [["1614", "Carcavelos (EstaÃ§Ã£o) - Portela de Sintra (EstaÃ§Ã£o)"], ["1619", "AbÃ³boda (Auto BarÃ£o) - Portela de Sintra (EstaÃ§Ã£o)"]],
        468: [["1615", "Carcavelos (EstaÃ§Ã£o) - Rio de Mouro (EstaÃ§Ã£o)"], ["1620", "AbÃ³boda (Auto BarÃ£o) - Rio de Mouro (EstaÃ§Ã£o)"]],
        470: [["1607", "ConceiÃ§Ã£o da AbÃ³boda - Oeiras (EstaÃ§Ã£o)"], ["1608", "Oeiras (EstaÃ§Ã£o) - Taguspark"], ["1609", "Oeiras (EstaÃ§Ã£o) - TalaÃ­de (Igreja)"]],
        471: [["1604", "Carcavelos (EstaÃ§Ã£o) - Parede (Terminal)"]],
        479: [["1604", "Carcavelos (EstaÃ§Ã£o) - Parede (Terminal)"]],
        Nova: [["1605", "Carnaxide (Av. JoÃ£o Paulo II) - Nova SBE"], ["1606", "Carnaxide (Av. JoÃ£o Paulo II) - Nova SBE, via Terrugem"], ["1612", "CacÃ©m (EstaÃ§Ã£o) - Carcavelos (EstaÃ§Ã£o)"]]
    },
    Lisboa: {
        1: [["1722", "Alfragide (Alegro) - Hospital SÃ£o Francisco Xavier"]],
        101: [["1717", "Lisboa (C. Militar) - Tercena, via Amadora Este (Metro)"]],
        107: [["1716", "Idanha - Lisboa (M. Pombal)"]],
        11: [["1724", "Linda-a-Velha - Lisboa (M. Pombal)"]],
        113: [["1714", "Amadora (EstaÃ§Ã£o Sul) - BelÃ©m (EstaÃ§Ã£o)"]],
        115: [["1725", "Lisboa (M. Pombal) - Oeiras (EstaÃ§Ã£o)"]],
        128: [["1701", "Alto Brandoa - Benfica | madrugada"], ["1702", "Alto Brandoa - Lisboa (C. Militar) | madrugada"], ["1706", "Lisboa (C. Militar) - UBBO"]],
        13: [["1726", "Lisboa (M. Pombal) - Queijas"], ["1728", "Lisboa (M. Pombal) - Queijas, via Linda-a-Velha"], ["1730", "Lisboa (M. Pombal) - Queluz Baixo, via Linda-a-Velha"]],
        132: [["1720", "Casal de Cambra - Lisboa (C. Militar), via Amadora"]],
        "13D": [["1727", "Lisboa (M. Pombal) - Queijas, via A5"], ["1729", "Lisboa (M. Pombal) - Queluz Baixo (C.C.)"]],
        142: [["1705", "Benfica (GrÃ£o Vasco) - UBBO, via Amadora Este (Metro) | noturna e madrugada"], ["1707", "Lisboa (C. Militar) - UBBO, via Falagueira"]],
        143: [["1703", "Amadora (EstaÃ§Ã£o Norte) - Pontinha (Metro)"]],
        144: [["1718", "CacÃ©m (Bairro Grajal) - BelÃ©m (EstaÃ§Ã£o)"]],
        149: [["1715", "BelÃ©m (EstaÃ§Ã£o) - Mira Sintra (Mercado)"]],
        15: [["1727", "Lisboa (M. Pombal) - Queijas, via A5"], ["1732", "Lisboa (M. Pombal) - SÃ£o Marcos, via Carnaxide"], ["1733", "Lisboa (M. Pombal) - SÃ£o Marcos, via Linda-a-Pastora"]],
        151: [["3704", "Charneca da Caparica - Lisboa (M. Pombal)"]],
        153: [["3710", "Costa da Caparica (Terminal) - Lisboa (Sete Rios)"]],
        155: [["3709", "Costa da Caparica (Terminal) - Lisboa (M. Pombal)"]],
        158: [["3711", "Monte da Caparica (FCT) - Lisboa (Sete Rios)"]],
        159: [["3707", "Charneca da Caparica - Lisboa (Sete Rios), via Sobreda"], ["3716", "Lisboa (Sete Rios) - Marisol"]],
        160: [["3703", "Almada (Parque Urbano) - Lisboa (Sete Rios)"]],
        161: [["3710", "Costa da Caparica (Terminal) - Lisboa (Sete Rios)"]],
        162: [["1713", "AlgÃ©s (Terminal) - Amadora Este (Metro)"], ["3717", "Lisboa (Sete Rios) - Quinta do Brasileiro"]],
        163: [["1721", "Lisboa (C. Militar) - MassamÃ¡ (Casal do Olival), via Amadora Este (Metro)"]],
        165: [["1708", "UBBO | Circular"]],
        169: [["3715", "Lisboa (M. Pombal) - Santa Marta do Pinhal"]],
        176: [["3702", "Almada (Parque Urbano) - Lisboa (C. UniversitÃ¡ria)"]],
        185: [["1704", "Amadora (Hospital) - Lisboa (M. Pombal)"]],
        "190 (adaptada)": [["3705", "Charneca da Caparica - Lisboa (Sete Rios)"]],
        20: [["1712", "AlgÃ©s (Terminal) - Amadora (EstaÃ§Ã£o Sul)"]],
        200: [["2740", "Ericeira (Terminal) - Lisboa (C. Grande), via A8"], ["2741", "Ericeira (Terminal) - Lisboa (C. Grande), via Ericeira (Centro), Freixeira e A8"], ["2804", "Mafra - Lisboa (C. Grande), via A8"]],
        201: [["2754", "Lisboa (C. Grande) - PÃ³voa da Galega"], ["2812", "CaneÃ§as (Esc. SecundÃ¡ria) - Lisboa (C. Grande), via Sr. Roubado (Metro)"]],
        202: [["2765", "CabeÃ§o de Montachique - Lisboa (C. Grande)"]],
        203: [["2830", "Casal Bispo - Lisboa (C. Militar)"], ["2831", "Casal Bispo - Pontinha (Metro)"]],
        204: [["2757", "Lisboa (C. Grande) - PÃ³voa da Galega, via Milharado e EN8"]],
        205: [["2819", "Lisboa (C. Militar) - Sr. Roubado (Metro)"], ["2824", "Pontinha (Metro) - Sr. Roubado (Metro)"]],
        206: [["2746", "Lisboa (C. Grande) - Venda do Pinheiro, via Milharado e A8"], ["2780", "Loures (C.C. Continente) - Pontinha (Metro)"], ["2820", "Odivelas (Colinas do Cruzeiro) - Pontinha (Metro)"], ["2821", "Odivelas (Metro) - Pontinha (Metro)"]],
        207: [["2801", "Ericeira (Terminal) - Lisboa (C. Grande), via A21/A8"], ["3721", "Lisboa (Sete Rios) - Sesimbra (Terminal)"]],
        208: [["2740", "Ericeira (Terminal) - Lisboa (C. Grande), via A8"], ["2741", "Ericeira (Terminal) - Lisboa (C. Grande), via Ericeira (Centro), Freixeira e A8"], ["2742", "Lisboa (C. Grande) - Mafra (Terminal)"], ["2751", "Ericeira (Terminal) - Lisboa (C. Grande)"], ["2758", "Mafra - Lisboa (C. Grande)"]],
        209: [["2753", "Lisboa (C. Grande) - Milharado"], ["2802", "Lisboa (C. Grande) - Mafra (Terminal), via A21"], ["2803", "Ericeira (Terminal) - Lisboa (C. Grande), via Mafra e A21/A8"]],
        210: [["1709", "CaneÃ§as - Lisboa (C. Militar)"], ["1710", "CaneÃ§as - Pontinha (Metro)"]],
        211: [["2816", "Lisboa (C. Grande) - Ramada (Bairro Bons Dias)"]],
        214: [["2768", "Casal Paradela - Lisboa (C. Grande)"]],
        219: [["2755", "Lisboa (C. Grande) - PÃ³voa da Galega, via Casais do Forno"]],
        220: [["2765", "CabeÃ§o de Montachique - Lisboa (C. Grande)"]],
        221: [["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        222: [["2811", "CaneÃ§as (Bairro do Monte Verde) - Lisboa (C. Militar)"], ["2823", "Pedernais (Bairro Girassol) - Pontinha (Metro)"]],
        223: [["2744", "Lisboa (C. Grande) - PÃ³voa da Galega, via Milharado e A8"], ["2813", "Casal Novo - Lisboa (C. Militar)"], ["2814", "Casal Novo - Pontinha (Metro)"]],
        224: [["1711", "CaneÃ§as (Esc. SecundÃ¡ria) - Pontinha (Metro)"]],
        225: [["2805", "EncarnaÃ§Ã£o - Lisboa (C. Grande)"]],
        226: [["2810", "Arroja - Lisboa (C. Grande)"]],
        227: [["2701", "Pontinha (Metro) - Vale Grande"]],
        228: [["2781", "Loures (C.C. Continente) - Pontinha (Metro), via Ramada"], ["2815", "Jardim da Amoreira - Pontinha (Metro)"], ["2818", "Lisboa (C. Militar) - Serra da Amoreira"]],
        229: [["2758", "Mafra - Lisboa (C. Grande)"], ["2807", "Lisboa (C. Grande) - Zambujal, via Mafra"]],
        230: [["2900", "Lisboa (C. Grande) - SÃ£o SebastiÃ£o"]],
        231: [["1709", "CaneÃ§as - Lisboa (C. Militar)"]],
        233: [["2745", "Lisboa (C. Grande) - PÃ³voa da Galega, via Murteira"], ["2850", "Costa da Caparica - PÃ³voa de Santo AdriÃ£o (Parque Urbano)"]],
        234: [["2743", "Lisboa (C. Grande) - PÃ³voa da Galega, via A8 - Loures"]],
        236: [["2832", "Casal Novo - Lisboa (C. Militar), via Casal de Cambra"], ["2833", "Pontinha (Metro) | Circular, via Casal Novo"]],
        237: [["2754", "Lisboa (C. Grande) - PÃ³voa da Galega"], ["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        239: [["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        246: [["2801", "Ericeira (Terminal) - Lisboa (C. Grande), via A21/A8"]],
        252: [["3720", "Lisboa (Sete Rios) - Quinta do Conde"]],
        3: [["2842", "Lisboa (C. Grande) - Vila Franca de Xira"]],
        300: [["2714", "Lisboa (C. Grande) - SacavÃ©m (Jardim)"]],
        301: [["2725", "Lisboa (Oriente) - Loures (C.C. Continente)"]],
        302: [["2732", "Lisboa (PÃ§ JosÃ© QueirÃ³s) | Circular, via SacavÃ©m e Camarate"]],
        305: [["2704", "Bairro Espinhal - Lisboa (Oriente)"], ["2727", "Lisboa (Oriente) - Loures, via Unhos"]],
        307: [["2735", "Urbana de SacavÃ©m | Circular, via Prior Velho"]],
        308: [["2731", "Lisboa (Oriente) - SacavÃ©m | Circular, via Portela"]],
        309: [["2708", "CabeÃ§o Aguieira - Lisboa (Oriente)"]],
        310: [["2711", "Charneca - Lisboa (Oriente)"], ["2712", "Charneca do Lumiar - SacavÃ©m (Jardim), via Bairro de Santiago"]],
        311: [["2710", "Catujal (Bairro Alto MoÃ­nho) - Lisboa (C. Grande)"]],
        312: [["2713", "Lisboa (C. Grande) | Circular, via SacavÃ©m e ApelaÃ§Ã£o"]],
        313: [["2713", "Lisboa (C. Grande) | Circular, via SacavÃ©m e ApelaÃ§Ã£o"]],
        315: [["2703", "Bairro de SÃ£o JosÃ© - Lisboa (C. Grande) | Circular"]],
        316: [["2730", "Lisboa (Oriente) - Santa Iria da AzÃ³ia"]],
        317: [["2728", "Bairro Covina - Lisboa (Oriente)"]],
        318: [["2729", "Lisboa (Oriente) - Portela da AzÃ³ia"]],
        319: [["2791", "Alverca (Z. Industrial) - Lisboa (C. Grande)"]],
        320: [["2790", "Alverca (EstaÃ§Ã£o) - Lisboa (C. Grande)"], ["2793", "Forte da Casa - Lisboa (C. Grande)"]],
        321: [["2722", "Lisboa (C. Grande) - Via Rara"]],
        329: [["2721", "Lisboa (C. Grande) - Periscoxe"], ["2795", "Lisboa (C. Grande) - Quinta da Piedade"]],
        330: [["2794", "Forte da Casa - Lisboa (Oriente)"]],
        331: [["2764", "Bucelas - Lisboa (C. Grande), via S. JulÃ£o do Tojal"], ["2767", "Casainhos - Lisboa (C. Grande), via Lumiar"]],
        333: [["2723", "Lisboa (C. Grande) - Zambujal"], ["4701", "Lisboa (Oriente) - Vale da Amoreira"]],
        334: [["2718", "Infantado - Lisboa (C. Grande)"]],
        335: [["2750", "Bucelas - Lisboa (C. Grande), via CabeÃ§o de Montachique e Odivelas (Metro)"], ["2766", "Casainhos - Lisboa (C. Grande)"], ["2767", "Casainhos - Lisboa (C. Grande), via Lumiar"], ["2779", "Lisboa (C. Grande) - Santo AntÃ£o do Tojal, via Infantado e Loures (C.Comercial)"]],
        336: [["2762", "Bucelas - Lisboa (C. Grande), via A8"], ["2763", "Bucelas - Lisboa (C. Grande), via Lumiar"], ["2767", "Casainhos - Lisboa (C. Grande), via Lumiar"]],
        337: [["2778", "Lisboa (C. Grande) - Santo AntÃ£o do Tojal"]],
        344: [["2706", "Bucelas - Lisboa (C. Grande)"], ["2707", "Bucelas - Lisboa (C. Grande), via SÃ£o JuliÃ£o do Tojal e A8"]],
        345: [["2792", "Arcena - Lisboa (Oriente)"]],
        347: [["2840", "Arcena - Lisboa (Oriente), via A1"]],
        353: [["2797", "Lisboa (C. Grande) - Vialonga, via A8"], ["2798", "Lisboa (C. Grande) - Vialonga, via A9 e A8"]],
        354: [["2796", "Lisboa (C. Grande) - Vialonga"]],
        410: [["2715", "Cidade Nova - Lisboa (C. Grande), via A8"]],
        411: [["2716", "Cidade Nova - Lisboa (C. Grande), via IC22"]],
        412: [["2769", "Cidade Nova - Lisboa (C. Grande)"]],
        413: [["2770", "Cidade Nova - Lisboa (C. Grande), via Lumiar"]],
        414: [["2771", "Cidade Nova - Lisboa (C. Grande), via Urb. Flores"]],
        415: [["2771", "Cidade Nova - Lisboa (C. Grande), via Urb. Flores"]],
        421: [["2717", "Lisboa (C. Grande) - Torres da Bela Vista, via IC22"]],
        422: [["2772", "Lisboa (C. Grande) - Torres da Bela Vista"]],
        423: [["2773", "Lisboa (C. Grande) - Torres da Bela Vista, via Lumiar"]],
        424: [["2772", "Lisboa (C. Grande) - Torres da Bela Vista"]],
        425: [["2773", "Lisboa (C. Grande) - Torres da Bela Vista, via Lumiar"]],
        426: [["2772", "Lisboa (C. Grande) - Torres da Bela Vista"]],
        427: [["2773", "Lisboa (C. Grande) - Torres da Bela Vista, via Lumiar"]],
        431: [["4703", "Lisboa (Oriente) - Montijo (Terminal RodoviÃ¡rio), via Alcochete e Samouco"]],
        432: [["4702", "Lisboa (Oriente) - Valbom"], ["4704", "Atalaia - Lisboa (Oriente)"]],
        435: [["4705", "Lisboa (Oriente) - Samouco"], ["4707", "Lisboa (Oriente) - Montijo (Terminal RodoviÃ¡rio)"]],
        450: [["2774", "Frielas - Lisboa (C. Grande)"]],
        451: [["2775", "Frielas - Lisboa (C. Grande), via Zona Industrial"]],
        453: [["4706", "SÃ£o Francisco - Lisboa (Oriente)"]],
        460: [["2720", "Lisboa (C. Grande) - Loures (Bairro Urmeira)"]],
        48: [["2842", "Lisboa (C. Grande) - Vila Franca de Xira"]],
        561: [["4725", "Lisboa (Sete Rios) - SetÃºbal (ITS)"]],
        562: [["4720", "Lisboa (Oriente) - SetÃºbal (ITS)"]],
        563: [["4715", "Lisboa (Oriente) - SetÃºbal (ITS), via Pinhal Novo"]],
        565: [["4710", "Lisboa (Oriente) - Palmela (Terminal)"], ["4711", "Lisboa (Oriente) - Pinhal Novo"]],
        "7/13": [["1723", "Carnaxide - Lisboa (M. Pombal)"]],
        701: [["2806", "Lisboa (C. Grande) - Livramento"]],
        702: [["2752", "Lisboa (C. Grande) - Malveira"]],
        72: [["2841", "Lisboa (C. Grande) - Sobralinho"]],
        750: [["2705", "Bairro Espinhal - Lisboa (Oriente) | Circular"]],
        810: [["2776", "Guerreiros - Lisboa (C. Grande)"]],
        811: [["2776", "Guerreiros - Lisboa (C. Grande)"]],
        812: [["2777", "Guerreiros - Lisboa (C. Grande), via Lumiar"]],
        813: [["2785", "Covas de Ferro - Lisboa (C. Grande)"]],
        814: [["2760", "Ã€-dos-Moninhos - Lisboa (C. Grande)"], ["2761", "Ã€-dos-Moninhos - Lisboa (C. Grande), via Bolores"]],
        815: [["2760", "Ã€-dos-Moninhos - Lisboa (C. Grande)"]],
        818: [["2785", "Covas de Ferro - Lisboa (C. Grande)"]],
        901: [["2812", "CaneÃ§as (Esc. SecundÃ¡ria) - Lisboa (C. Grande), via Sr. Roubado (Metro)"]],
        905: [["2817", "Lisboa (C. Militar) - Odivelas (Metro)"], ["2822", "Odivelas (Metro) - Pontinha (Metro), via Serra da Luz"]],
        931: [["2702", "Lisboa (C. Grande) - Pontinha (Metro)"]],
        Nova: [["1719", "Casal de Cambra - Lisboa (C. Militar)"], ["1731", "CacÃ©m (EstaÃ§Ã£o) - Hospital SÃ£o Francisco Xavier"], ["2709", "Camarate | Circular"], ["2719", "Lisboa (C. Grande) - Loures"], ["2724", "Lisboa (Oriente) - Loures"], ["2726", "Lisboa (Oriente) - Loures, via SacavÃ©m"], ["2733", "Loures - Moscavide (Metro)"], ["2734", "Prior Velho - SacavÃ©m (EstaÃ§Ã£o)"], ["3701", "Almada (Centro Sul) - AlgÃ©s (Terminal)"], ["3706", "Charneca da Caparica - Lisboa (Sete Rios), via FeijÃ³"], ["3708", "Costa da Caparica (Terminal) - Lisboa (C. SodrÃ©)"]]
    },
    Loures: {
        200: [["2740", "Ericeira (Terminal) - Lisboa (C. Grande), via A8"], ["2741", "Ericeira (Terminal) - Lisboa (C. Grande), via Ericeira (Centro), Freixeira e A8"]],
        201: [["2754", "Lisboa (C. Grande) - PÃ³voa da Galega"]],
        202: [["2522", "Montemor - Sr. Roubado (Metro)"], ["2765", "CabeÃ§o de Montachique - Lisboa (C. Grande)"]],
        204: [["2757", "Lisboa (C. Grande) - PÃ³voa da Galega, via Milharado e EN8"]],
        206: [["2519", "Loures (C.C. Continente) - Odivelas (Colinas do Cruzeiro)"], ["2521", "Loures (Campo de Jogos) - Odivelas (Metro)"], ["2601", "Loures (C.C. Continente) - Reboleira (Metro)"], ["2746", "Lisboa (C. Grande) - Venda do Pinheiro, via Milharado e A8"], ["2780", "Loures (C.C. Continente) - Pontinha (Metro)"]],
        207: [["2504", "Carrascal - Ponte de Lousa"]],
        208: [["2740", "Ericeira (Terminal) - Lisboa (C. Grande), via A8"], ["2741", "Ericeira (Terminal) - Lisboa (C. Grande), via Ericeira (Centro), Freixeira e A8"], ["2742", "Lisboa (C. Grande) - Mafra (Terminal)"], ["2751", "Ericeira (Terminal) - Lisboa (C. Grande)"], ["2758", "Mafra - Lisboa (C. Grande)"]],
        209: [["2753", "Lisboa (C. Grande) - Milharado"]],
        210: [["2501", "Bocal - Malveira"]],
        211: [["2009", "CabeÃ§o de Montachique - Loures (Centro SaÃºde)"], ["2011", "CabeÃ§o de Montachique - Loures (Centro SaÃºde), via Murteira"]],
        214: [["2515", "Casal Paradela - Odivelas (Metro)"], ["2768", "Casal Paradela - Lisboa (C. Grande)"]],
        215: [["2513", "CaneÃ§as (Esc. SecundÃ¡ria) - Loures (Lg Marcos RomÃ£o Reis JÃºnior)"], ["2605", "CacÃ©m (EstaÃ§Ã£o) - Loures (Lg Marcos RomÃ£o Reis JÃºnior)"], ["2901", "Rolia - Venda do Pinheiro (Terminal)"]],
        219: [["2755", "Lisboa (C. Grande) - PÃ³voa da Galega, via Casais do Forno"]],
        220: [["2010", "CabeÃ§o de Montachique - Loures (Centro SaÃºde), via Bairro Novo Palhais"], ["2765", "CabeÃ§o de Montachique - Lisboa (C. Grande)"]],
        221: [["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        223: [["2744", "Lisboa (C. Grande) - PÃ³voa da Galega, via Milharado e A8"]],
        225: [["2524", "Odivelas (Metro) | Circular, via Hospital Beatriz Ã‚ngelo"]],
        228: [["2781", "Loures (C.C. Continente) - Pontinha (Metro), via Ramada"]],
        229: [["2758", "Mafra - Lisboa (C. Grande)"]],
        233: [["2745", "Lisboa (C. Grande) - PÃ³voa da Galega, via Murteira"]],
        234: [["2743", "Lisboa (C. Grande) - PÃ³voa da Galega, via A8 - Loures"]],
        235: [["2516", "Casal Paradela - Sr. Roubado (Metro)"]],
        237: [["2754", "Lisboa (C. Grande) - PÃ³voa da Galega"], ["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        238: [["2518", "IKEA Loures - Sr. Roubado (Metro)"]],
        239: [["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        300: [["2714", "Lisboa (C. Grande) - SacavÃ©m (Jardim)"]],
        301: [["2023", "Loures (C.C. Continente) - SacavÃ©m (ClÃ­nica Sto AntÃ³nio)"], ["2024", "Loures (EDP) - SacavÃ©m (ClÃ­nica Sto AntÃ³nio)"], ["2725", "Lisboa (Oriente) - Loures (C.C. Continente)"]],
        302: [["2732", "Lisboa (PÃ§ JosÃ© QueirÃ³s) | Circular, via SacavÃ©m e Camarate"]],
        303: [["2029", "Moscavide (Metro) | Circular, via Portela"]],
        305: [["2005", "Bairro do Espinhal - SacavÃ©m (EstaÃ§Ã£o)"], ["2025", "Loures (Lg Marcos RomÃ£o Reis JÃºnior) - SacavÃ©m (EstaÃ§Ã£o)"], ["2704", "Bairro Espinhal - Lisboa (Oriente)"], ["2727", "Lisboa (Oriente) - Loures, via Unhos"]],
        306: [["2036", "Urbana de Camarate | Circular"]],
        307: [["2735", "Urbana de SacavÃ©m | Circular, via Prior Velho"]],
        308: [["2731", "Lisboa (Oriente) - SacavÃ©m | Circular, via Portela"]],
        309: [["2708", "CabeÃ§o Aguieira - Lisboa (Oriente)"]],
        310: [["2711", "Charneca - Lisboa (Oriente)"], ["2712", "Charneca do Lumiar - SacavÃ©m (Jardim), via Bairro de Santiago"]],
        311: [["2710", "Catujal (Bairro Alto MoÃ­nho) - Lisboa (C. Grande)"]],
        312: [["2713", "Lisboa (C. Grande) | Circular, via SacavÃ©m e ApelaÃ§Ã£o"]],
        313: [["2713", "Lisboa (C. Grande) | Circular, via SacavÃ©m e ApelaÃ§Ã£o"]],
        315: [["2703", "Bairro de SÃ£o JosÃ© - Lisboa (C. Grande) | Circular"]],
        316: [["2730", "Lisboa (Oriente) - Santa Iria da AzÃ³ia"]],
        317: [["2728", "Bairro Covina - Lisboa (Oriente)"]],
        318: [["2729", "Lisboa (Oriente) - Portela da AzÃ³ia"]],
        319: [["2791", "Alverca (Z. Industrial) - Lisboa (C. Grande)"]],
        320: [["2790", "Alverca (EstaÃ§Ã£o) - Lisboa (C. Grande)"], ["2793", "Forte da Casa - Lisboa (C. Grande)"]],
        321: [["2722", "Lisboa (C. Grande) - Via Rara"]],
        325: [["2536", "PÃ³voa de Santa Iria - SalvaÃ§Ã£o | Circular"]],
        "327 (adaptada)": [["2026", "Loures | Circular"]],
        329: [["2721", "Lisboa (C. Grande) - Periscoxe"], ["2795", "Lisboa (C. Grande) - Quinta da Piedade"]],
        330: [["2794", "Forte da Casa - Lisboa (Oriente)"]],
        331: [["2764", "Bucelas - Lisboa (C. Grande), via S. JulÃ£o do Tojal"], ["2767", "Casainhos - Lisboa (C. Grande), via Lumiar"]],
        332: [["2037", "Zambujal | Circular"]],
        333: [["2723", "Lisboa (C. Grande) - Zambujal"]],
        334: [["2718", "Infantado - Lisboa (C. Grande)"]],
        335: [["2503", "Bucelas - Santo AntÃ£o do Tojal"], ["2510", "Bucelas - Sr. Roubado (Metro)"], ["2514", "Casainhos - Sr. Roubado (Metro), via Loures (C.Comercial)"], ["2750", "Bucelas - Lisboa (C. Grande), via CabeÃ§o de Montachique e Odivelas (Metro)"], ["2766", "Casainhos - Lisboa (C. Grande)"], ["2767", "Casainhos - Lisboa (C. Grande), via Lumiar"], ["2779", "Lisboa (C. Grande) - Santo AntÃ£o do Tojal, via Infantado e Loures (C.Comercial)"]],
        336: [["2762", "Bucelas - Lisboa (C. Grande), via A8"], ["2763", "Bucelas - Lisboa (C. Grande), via Lumiar"], ["2767", "Casainhos - Lisboa (C. Grande), via Lumiar"]],
        337: [["2021", "Infantado - Santo AntÃ£o do Tojal"], ["2022", "Loures - Monte EsperanÃ§a"], ["2525", "Santo AntÃ£o do Tojal - Sr. Roubado (Metro)"], ["2778", "Lisboa (C. Grande) - Santo AntÃ£o do Tojal"]],
        338: [["2021", "Infantado - Santo AntÃ£o do Tojal"]],
        340: [["2502", "Bucelas - Malveira"]],
        342: [["2007", "Bucelas - Mato da Cruz"], ["2530", "Alverca (EstaÃ§Ã£o) - Bucelas, via Malvarosa"]],
        344: [["2706", "Bucelas - Lisboa (C. Grande)"], ["2707", "Bucelas - Lisboa (C. Grande), via SÃ£o JuliÃ£o do Tojal e A8"]],
        345: [["2792", "Arcena - Lisboa (Oriente)"]],
        346: [["2531", "Alverca (EstaÃ§Ã£o) - Granja, via Casal das Areias"], ["2533", "Alverca (EstaÃ§Ã£o) - Santo AntÃ£o do Tojal, via Casal das Areias"]],
        348: [["2537", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Santo AntÃ£o do Tojal"]],
        353: [["2797", "Lisboa (C. Grande) - Vialonga, via A8"], ["2798", "Lisboa (C. Grande) - Vialonga, via A9 e A8"]],
        354: [["2796", "Lisboa (C. Grande) - Vialonga"]],
        355: [["2651", "Bucelas - Costa da Caparica (Terminal)"]],
        357: [["2008", "Bucelas - RomÃ£o Charneca | Circular"]],
        358: [["2006", "Bemposta - Bucelas | Circular"]],
        360: [["2532", "Alverca (EstaÃ§Ã£o) - Loures"]],
        363: [["2035", "Santo AntÃ£o do Tojal | Circular"]],
        365: [["2520", "Loures (C.Comercial) - Sr. Roubado (Metro)"]],
        370: [["2537", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Santo AntÃ£o do Tojal"], ["2538", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Santo AntÃ£o do Tojal, via Zambujal"]],
        410: [["2715", "Cidade Nova - Lisboa (C. Grande), via A8"]],
        411: [["2716", "Cidade Nova - Lisboa (C. Grande), via IC22"]],
        412: [["2769", "Cidade Nova - Lisboa (C. Grande)"]],
        413: [["2770", "Cidade Nova - Lisboa (C. Grande), via Lumiar"]],
        414: [["2771", "Cidade Nova - Lisboa (C. Grande), via Urb. Flores"]],
        415: [["2771", "Cidade Nova - Lisboa (C. Grande), via Urb. Flores"]],
        416: [["2012", "Conventinho - Sto. Ant. Cavaleiros"], ["2020", "Sto. Ant. Cavaleiros | Circular, via Hospital Beatriz Ã‚ngelo"]],
        417: [["2014", "Escola Maria Veleda - Frielas"]],
        421: [["2717", "Lisboa (C. Grande) - Torres da Bela Vista, via IC22"]],
        422: [["2772", "Lisboa (C. Grande) - Torres da Bela Vista"]],
        423: [["2773", "Lisboa (C. Grande) - Torres da Bela Vista, via Lumiar"]],
        424: [["2772", "Lisboa (C. Grande) - Torres da Bela Vista"]],
        425: [["2773", "Lisboa (C. Grande) - Torres da Bela Vista, via Lumiar"]],
        426: [["2019", "Hospital Beatriz Ã‚ngelo - Torres da Bela Vista, via Escola JosÃ© Cardoso Pires"], ["2772", "Lisboa (C. Grande) - Torres da Bela Vista"]],
        427: [["2018", "Hospital Beatriz Ã‚ngelo - Torres da Bela Vista"], ["2773", "Lisboa (C. Grande) - Torres da Bela Vista, via Lumiar"]],
        430: [["2015", "Flamenga - Torres da Bela Vista"]],
        431: [["2013", "Conventinho - Torres da Bela Vista"]],
        440: [["2027", "Loures | Circular, via Cidade Nova"]],
        441: [["2028", "Loures | Circular, via Ponte Frielas"]],
        450: [["2774", "Frielas - Lisboa (C. Grande)"]],
        451: [["2775", "Frielas - Lisboa (C. Grande), via Zona Industrial"]],
        460: [["2720", "Lisboa (C. Grande) - Loures (Bairro Urmeira)"]],
        470: [["2650", "Cidade Nova - Costa da Caparica"]],
        701: [["2652", "Costa da Caparica (Terminal) - Forte da Casa"]],
        702: [["2752", "Lisboa (C. Grande) - Malveira"]],
        710: [["2031", "SacavÃ©m (C. SaÃºde) | Circular"]],
        711: [["2030", "SacavÃ©m (C. SaÃºde) - Fetais | Circular"]],
        750: [["2705", "Bairro Espinhal - Lisboa (Oriente) | Circular"]],
        810: [["2776", "Guerreiros - Lisboa (C. Grande)"]],
        811: [["2017", "Guerreiros - Loures (Campo de Jogos)"], ["2776", "Guerreiros - Lisboa (C. Grande)"]],
        812: [["2777", "Guerreiros - Lisboa (C. Grande), via Lumiar"]],
        813: [["2785", "Covas de Ferro - Lisboa (C. Grande)"]],
        814: [["2003", "Ã€-dos-Moninhos - Guerreiros"], ["2760", "Ã€-dos-Moninhos - Lisboa (C. Grande)"], ["2761", "Ã€-dos-Moninhos - Lisboa (C. Grande), via Bolores"]],
        815: [["2760", "Ã€-dos-Moninhos - Lisboa (C. Grande)"]],
        816: [["2621", "Covas de Ferro - Pinheiro de Loures"]],
        818: [["2002", "A-dos-CÃ£os - Loures (Centro SaÃºde)"], ["2785", "Covas de Ferro - Lisboa (C. Grande)"]],
        819: [["2001", "A-Dos-CÃ£os - Loures (Campo de Jogos)"]],
        822: [["2620", "Covas de Ferro - Loures (Centro SaÃºde), via A-Dos-Calvos"]],
        824: [["2016", "Guerreiros - Hospital Beatriz Ã‚ngelo"]],
        826: [["2004", "Bairro de Santa Maria - Loures (Centro SaÃºde)"]],
        831: [["2620", "Covas de Ferro - Loures (Centro SaÃºde), via A-Dos-Calvos"]],
        925: [["2517", "Hospital Beatriz Ã‚ngelo - Odivelas (Metro) | Circular"]],
        934: [["2523", "Odivelas (Metro) - Montemor"]],
        Nova: [["2032", "SacavÃ©m (EstaÃ§Ã£o) - Santa Iria da AzÃ³ia (EstaÃ§Ã£o)"], ["2033", "Bobadela - SÃ£o JoÃ£o da Talha | Circular"], ["2034", "Santa Iria da AzÃ³ia | Circular"], ["2505", "Loures (C. Comercial) - Malveira"], ["2506", "Milharado (CASO) - Ponte de Lousa"], ["2511", "Bairro dos CTT - Loures (C. Comercial)"], ["2512", "Bucelas - Sr. Roubado (Metro), via Ramada"], ["2534", "Loures - Santa Iria da AzÃ³ia"], ["2535", "Loures - Vila Franca de Xira"], ["2539", "SacavÃ©m (EstaÃ§Ã£o) - Santa Iria da AzÃ³ia (EstaÃ§Ã£o), via Portela da AzÃ³ia"], ["2540", "Santa Iria da AzÃ³ia - Vialonga"], ["2709", "Camarate | Circular"], ["2719", "Lisboa (C. Grande) - Loures"], ["2724", "Lisboa (Oriente) - Loures"], ["2726", "Lisboa (Oriente) - Loures, via SacavÃ©m"], ["2733", "Loures - Moscavide (Metro)"], ["2734", "Prior Velho - SacavÃ©m (EstaÃ§Ã£o)"]],
        w: [["2003", "Ã€-dos-Moninhos - Guerreiros"]]
    },
    Mafra: {
        124: [["1634", "Fonte da Aranha - Montelavar"]],
        200: [["2740", "Ericeira (Terminal) - Lisboa (C. Grande), via A8"], ["2741", "Ericeira (Terminal) - Lisboa (C. Grande), via Ericeira (Centro), Freixeira e A8"], ["2804", "Mafra - Lisboa (C. Grande), via A8"]],
        201: [["2754", "Lisboa (C. Grande) - PÃ³voa da Galega"]],
        202: [["1633", "Ericeira (Terminal) - Portela de Sintra (EstaÃ§Ã£o)"]],
        203: [["2128", "Ericeira (Terminal) - Fonte Boa da Brincosa"], ["2133", "Ericeira (Terminal) - Mafra (PalÃ¡cio), via Carvoeira, Montesouros e Av. Portugal"], ["2143", "Mafra (PalÃ¡cio) - Zambujal"]],
        204: [["2112", "Casais de SÃ£o LourenÃ§o - Ericeira (Terminal)"], ["2127", "Ericeira (Terminal) - Barril"], ["2757", "Lisboa (C. Grande) - PÃ³voa da Galega, via Milharado e EN8"], ["2905", "Aranha (Rotunda) - Ericeira (Terminal)"], ["2907", "Assenta - Ericeira (Terminal)"]],
        205: [["2118", "EncarnaÃ§Ã£o (CemitÃ©rio) - Ericeira (Terminal), via Santo Isidoro e Monte Godel"]],
        206: [["2113", "Casais de SÃ£o LourenÃ§o - Ericeira (Terminal), via Feiteira"], ["2117", "EncarnaÃ§Ã£o (CemitÃ©rio) - Ericeira (Terminal), via Ribamar"], ["2746", "Lisboa (C. Grande) - Venda do Pinheiro, via Milharado e A8"]],
        207: [["2504", "Carrascal - Ponte de Lousa"], ["2801", "Ericeira (Terminal) - Lisboa (C. Grande), via A21/A8"]],
        208: [["2132", "Ericeira (Terminal) - Mafra"], ["2740", "Ericeira (Terminal) - Lisboa (C. Grande), via A8"], ["2741", "Ericeira (Terminal) - Lisboa (C. Grande), via Ericeira (Centro), Freixeira e A8"], ["2742", "Lisboa (C. Grande) - Mafra (Terminal)"], ["2751", "Ericeira (Terminal) - Lisboa (C. Grande)"], ["2758", "Mafra - Lisboa (C. Grande)"]],
        209: [["2753", "Lisboa (C. Grande) - Milharado"], ["2802", "Lisboa (C. Grande) - Mafra (Terminal), via A21"], ["2803", "Ericeira (Terminal) - Lisboa (C. Grande), via Mafra e A21/A8"]],
        210: [["2125", "Urbana da Ericeira 2"], ["2501", "Bocal - Malveira"]],
        211: [["2124", "Urbana da Ericeira 1"]],
        215: [["2102", "A-da-Perra - Lagoa (ColÃ©gio Miramar)"], ["2137", "Lagoa (ColÃ©gio Miramar) - Mafra (PalÃ¡cio)"], ["2140", "Lagoa (ColÃ©gio Miramar) - Sobreiro"], ["2149", "Prezinheira - Venda do Pinheiro (Terminal)"], ["2901", "Rolia - Venda do Pinheiro (Terminal)"]],
        216: [["2101", "Achada - Lagoa (ColÃ©gio Miramar)"], ["2108", "Cachoeira - Malveira"], ["2109", "Cachoeira - Venda do Pinheiro (Terminal)"], ["2129", "Ericeira (Terminal) - Lagoa (ColÃ©gio Miramar)"], ["2130", "Ericeira (Terminal) - Lagoa (ColÃ©gio Miramar), via Ribamar"], ["2131", "Ericeira (Terminal) - Lagoa (ColÃ©gio Miramar), via Santo Isidoro"], ["2147", "Milharado - Venda do Pinheiro (Terminal)"]],
        217: [["2111", "Casais da Areia - Lagoa (ColÃ©gio Miramar)"], ["2114", "Charneca - Lagoa (ColÃ©gio Miramar)"], ["2119", "EncarnaÃ§Ã£o (CemitÃ©rio) - Lagoa (ColÃ©gio Miramar)"], ["2148", "PÃ³voa da Galega - Venda do Pinheiro (Terminal)"], ["2151", "Venda do Pinheiro (Terminal) - Vila de Canas"]],
        218: [["2138", "Lagoa (ColÃ©gio Miramar) - SÃ£o Domingos"]],
        219: [["2115", "CodeÃ§al (Tapada Nacional) - Lagoa (ColÃ©gio Miramar)"], ["2139", "Lagoa (ColÃ©gio Miramar) - Sobral da Abelheira"], ["2755", "Lisboa (C. Grande) - PÃ³voa da Galega, via Casais do Forno"]],
        220: [["2626", "Mafra (Parque Desportivo) - Portela de Sintra (EstaÃ§Ã£o)"]],
        221: [["2105", "Barreiralva (Igreja) - Mafra (Parque Desportivo)"], ["2116", "EncarnaÃ§Ã£o - Mafra (Parque Desportivo)"], ["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"], ["2914", "Mafra (Parque Desportivo) - SÃ£o Pedro da Cadeira (Rotunda da Aranha)"]],
        222: [["2103", "Antas - Mafra (Parque Desportivo)"]],
        223: [["2144", "Mafra (Parque Desportivo) - Livramento"], ["2744", "Lisboa (C. Grande) - PÃ³voa da Galega, via Milharado e A8"]],
        224: [["2134", "Ervideira - Gradil"], ["2913", "Mafra (Parque Desportivo) - Pero Negro (EstaÃ§Ã£o)"]],
        225: [["2116", "EncarnaÃ§Ã£o - Mafra (Parque Desportivo)"], ["2805", "EncarnaÃ§Ã£o - Lisboa (C. Grande)"]],
        226: [["2104", "Avessada - Malveira"], ["2106", "Bocal - Mafra (Parque Desportivo)"], ["2107", "Bocal - Malveira, via Avessada e Portela"]],
        227: [["2110", "Carvalhal - Mafra (Parque Desportivo), via Mata Grande e Valverde"]],
        229: [["2758", "Mafra - Lisboa (C. Grande)"], ["2807", "Lisboa (C. Grande) - Zambujal, via Mafra"]],
        230: [["2900", "Lisboa (C. Grande) - SÃ£o SebastiÃ£o"], ["2915", "Malveira - Vila Franca do RosÃ¡rio"]],
        231: [["2910", "Ervideira - Malveira"], ["2911", "Gradil - Malveira"], ["2912", "Gradil - Malveira (Terminal)"]],
        233: [["2145", "Mafra | Circular, via Sobral da Abelheira"], ["2745", "Lisboa (C. Grande) - PÃ³voa da Galega, via Murteira"]],
        234: [["2743", "Lisboa (C. Grande) - PÃ³voa da Galega, via A8 - Loures"]],
        237: [["2754", "Lisboa (C. Grande) - PÃ³voa da Galega"], ["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        238: [["2150", "Valverde - Venda do Pinheiro"]],
        239: [["2135", "Igreja Nova - Venda do Pinheiro (NÃºcleo Empresarial sul)"], ["2627", "Mafra (Parque Desportivo) - Ribeira dos TostÃµes"], ["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        241: [["2146", "Malveira (Centro SaÃºde) - Milharado (CASO) | Circular"], ["2906", "Assenta - EncarnaÃ§Ã£o"], ["2908", "Cambelas - EncarnaÃ§Ã£o"], ["2909", "Cambelas - Freiria (E.B. 2-3)"]],
        246: [["2801", "Ericeira (Terminal) - Lisboa (C. Grande), via A21/A8"]],
        248: [["2123", "Praia da Foz do Lizandro - Praia de Ribeira d'Ilhas"]],
        252: [["2142", "Mafra (Almada) | Circular"]],
        281: [["2126", "Ericeira | Circular"]],
        335: [["2503", "Bucelas - Santo AntÃ£o do Tojal"], ["2510", "Bucelas - Sr. Roubado (Metro)"], ["2750", "Bucelas - Lisboa (C. Grande), via CabeÃ§o de Montachique e Odivelas (Metro)"]],
        340: [["2502", "Bucelas - Malveira"]],
        445: [["1632", "Carvalhal - Portela de Sintra (EstaÃ§Ã£o)"]],
        701: [["2806", "Lisboa (C. Grande) - Livramento"]],
        702: [["2752", "Lisboa (C. Grande) - Malveira"]],
        721: [["2122", "Enxara do Bispo - Livramento"], ["2913", "Mafra (Parque Desportivo) - Pero Negro (EstaÃ§Ã£o)"]],
        Nova: [["2120", "Enxara do Bispo - Ericeira (Terminal)"], ["2121", "Enxara do Bispo - Gradil"], ["2136", "Jerumelo - Mafra"], ["2141", "Mafra - Ribamar"], ["2505", "Loures (C. Comercial) - Malveira"], ["2506", "Milharado (CASO) - Ponte de Lousa"], ["2625", "Mafra - SÃ£o JoÃ£o das Lampas"]]
    },
    Moita: {
        245: [["3650", "Moita - Sesimbra (Terminal)"]],
        302: [["4620", "Moita - Paio Pires"]],
        305: [["4610", "Bairro dos Marinheiros - Barreiro (Terminal)"]],
        "307 (Adaptado)": [["4604", "Barreiro (Terminal) - Moita (Escola Fragata do Tejo)"]],
        "311 (Adaptado)": [["4611", "Penalva - Moita (Esc. SecundÃ¡ria)"]],
        "312 (Adaptado)": [["4102", "CabeÃ§o Verde - Sarilhos Pequenos"]],
        "313 (Adaptado)": [["4104", "Moita | Circular"], ["4532", "Moita - Quatro Marcos"]],
        317: [["4602", "Alhos Vedros (EstaÃ§Ã£o) - Barreiro (Terminal)"]],
        318: [["4103", "Moita (EstaÃ§Ã£o) - Sarilhos Pequenos"]],
        "326 (Adaptado)": [["4531", "Moita - Palmela (Terminal)"]],
        330: [["4605", "Lavradio - Pinhal do Forno"]],
        333: [["4701", "Lisboa (Oriente) - Vale da Amoreira"]],
        "336 (Adaptado)": [["4101", "Alhos Vedros (Escola JosÃ© Afonso) - Arroteias"]],
        410: [["4600", "Alcochete (Freeport) - Barreiro (Terminal)"], ["4601", "Barreiro (Terminal) - Montijo (Terminal RodoviÃ¡rio)"], ["4603", "Barreiro (Terminal) - ChÃ£o Duro"]],
        451: [["4530", "Bairro Vila Morena - Pinhal Novo"]],
        Nova: [["4621", "Moita - Seixal (Terminal Fluvial)"]]
    },
    Montijo: {
        333: [["4701", "Lisboa (Oriente) - Vale da Amoreira"]],
        401: [["4204", "Bairro do CharqueirÃ£o - Montijo (Terminal Fluvial)"], ["4205", "Bairro do CharqueirÃ£o - Montijo (Terminal Fluvial), via Vale Porim"], ["4207", "Montijo (Ã�rea Comercial) - Montijo (Terminal Fluvial)"]],
        403: [["4203", "Afonsoeiro - Montijo (Terminal Fluvial), via Bairro da Liberdade"], ["4206", "Bairro Esteval - Montijo (Terminal Fluvial)"]],
        404: [["4202", "Afonsoeiro - Bairro do Saldanha, via Bairro da CalÃ§ada"]],
        410: [["4511", "Alcochete (Freeport) - Montijo (Terminal RodoviÃ¡rio), via Samouco"], ["4600", "Alcochete (Freeport) - Barreiro (Terminal)"], ["4601", "Barreiro (Terminal) - Montijo (Terminal RodoviÃ¡rio)"]],
        "412 (Adaptado)": [["4501", "Alcochete - Montijo (Terminal Fluvial)"]],
        413: [["4510", "Alcochete (Freeport) - Montijo (Terminal RodoviÃ¡rio)"], ["4512", "Alcochete (Freeport) - SetÃºbal (ITS), via Alto Estanqueiro"], ["4513", "Alcochete (Freeport) - Pinhal Novo"], ["4517", "Montijo (Terminal RodoviÃ¡rio) - SetÃºbal (ITS)"], ["4523", "Montijo (Terminal RodoviÃ¡rio) - Pinhal Novo"]],
        414: [["4514", "Canha - Montijo (Terminal RodoviÃ¡rio), via PegÃµes"]],
        415: [["4504", "Montijo (Terminal Fluvial) - Passil"]],
        416: [["4514", "Canha - Montijo (Terminal RodoviÃ¡rio), via PegÃµes"], ["4515", "Montijo (Terminal RodoviÃ¡rio) - PegÃµes"]],
        419: [["4502", "Alcochete - Passil"]],
        426: [["4516", "Montijo (Terminal RodoviÃ¡rio) - Rio Frio"]],
        431: [["4703", "Lisboa (Oriente) - Montijo (Terminal RodoviÃ¡rio), via Alcochete e Samouco"]],
        432: [["4704", "Atalaia - Lisboa (Oriente)"]],
        435: [["4705", "Lisboa (Oriente) - Samouco"], ["4707", "Lisboa (Oriente) - Montijo (Terminal RodoviÃ¡rio)"]],
        440: [["4643", "Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)"]],
        441: [["4210", "Canha - Foros Boavista"], ["4212", "Foros Boavista - PegÃµes"]],
        444: [["4520", "Faias - PegÃµes"]],
        446: [["4521", "Faias - Pinhal Novo"], ["4522", "Faias - PoceirÃ£o"]],
        447: [["4524", "Palmela (Terminal) - PegÃµes"]],
        451: [["4530", "Bairro Vila Morena - Pinhal Novo"]],
        453: [["4201", "Afonsoeiro - Bairro da Liberdade"], ["4706", "SÃ£o Francisco - Lisboa (Oriente)"]],
        8080: [["4905", "Faias - Vendas Novas"]],
        8902: [["4902", "Landeira - PegÃµes"], ["4906", "SetÃºbal (ITS) - Vendas Novas, via Landeira"]],
        Nova: [["4208", "Montijo (Terminal RodoviÃ¡rio) - Sarilhos Grandes (Estr. 4 Marcos)"], ["4211", "Craveiras - PegÃµes | Circular"], ["4503", "Atalaia - Jardia"]]
    },
    Odivelas: {
        "1/3": [["2217", "Odivelas (Metro) | Circular"]],
        133: [["1603", "Amadora (EstaÃ§Ã£o Norte) - CaneÃ§as"]],
        201: [["2207", "CaneÃ§as (Esc. SecundÃ¡ria) - Sr. Roubado (Metro), via Odivelas (Centro)"], ["2754", "Lisboa (C. Grande) - PÃ³voa da Galega"], ["2812", "CaneÃ§as (Esc. SecundÃ¡ria) - Lisboa (C. Grande), via Sr. Roubado (Metro)"]],
        202: [["2522", "Montemor - Sr. Roubado (Metro)"], ["2765", "CabeÃ§o de Montachique - Lisboa (C. Grande)"]],
        203: [["2830", "Casal Bispo - Lisboa (C. Militar)"], ["2831", "Casal Bispo - Pontinha (Metro)"]],
        204: [["2757", "Lisboa (C. Grande) - PÃ³voa da Galega, via Milharado e EN8"]],
        205: [["2819", "Lisboa (C. Militar) - Sr. Roubado (Metro)"], ["2824", "Pontinha (Metro) - Sr. Roubado (Metro)"]],
        206: [["2519", "Loures (C.C. Continente) - Odivelas (Colinas do Cruzeiro)"], ["2521", "Loures (Campo de Jogos) - Odivelas (Metro)"], ["2601", "Loures (C.C. Continente) - Reboleira (Metro)"], ["2780", "Loures (C.C. Continente) - Pontinha (Metro)"], ["2820", "Odivelas (Colinas do Cruzeiro) - Pontinha (Metro)"], ["2821", "Odivelas (Metro) - Pontinha (Metro)"]],
        207: [["2219", "Odivelas (Metro) | Circular, via PÃ³voa de Santo AdriÃ£o"]],
        208: [["2203", "Arroja | Circular, via Odivelas (Metro)"], ["2751", "Ericeira (Terminal) - Lisboa (C. Grande)"], ["2758", "Mafra - Lisboa (C. Grande)"]],
        209: [["2203", "Arroja | Circular, via Odivelas (Metro)"], ["2753", "Lisboa (C. Grande) - Milharado"]],
        210: [["1709", "CaneÃ§as - Lisboa (C. Militar)"], ["1710", "CaneÃ§as - Pontinha (Metro)"]],
        211: [["2215", "Odivelas (Metro) - Ramada (Bairro Bons Dias)"], ["2816", "Lisboa (C. Grande) - Ramada (Bairro Bons Dias)"]],
        212: [["2208", "CaneÃ§as (Jardim) - Vale Nogueira"]],
        213: [["2206", "CaneÃ§as (Esc. SecundÃ¡ria) - Sr. Roubado (Metro)"]],
        214: [["2515", "Casal Paradela - Odivelas (Metro)"], ["2768", "Casal Paradela - Lisboa (C. Grande)"]],
        215: [["2513", "CaneÃ§as (Esc. SecundÃ¡ria) - Loures (Lg Marcos RomÃ£o Reis JÃºnior)"], ["2605", "CacÃ©m (EstaÃ§Ã£o) - Loures (Lg Marcos RomÃ£o Reis JÃºnior)"], ["2606", "CacÃ©m (EstaÃ§Ã£o) - CaneÃ§as (Casa da Cultura)"]],
        216: [["2223", "Sr. Roubado (Metro) | Circular, via Casal Novo"]],
        219: [["2755", "Lisboa (C. Grande) - PÃ³voa da Galega, via Casais do Forno"]],
        220: [["1636", "Bairro Arco Maria Teresa - Dona Maria"], ["1637", "Casal de Cambra - Vale de Lobos"], ["2765", "CabeÃ§o de Montachique - Lisboa (C. Grande)"]],
        221: [["1635", "Almargem do Bispo (Centro de SaÃºde) - CaneÃ§as"], ["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        222: [["2811", "CaneÃ§as (Bairro do Monte Verde) - Lisboa (C. Militar)"], ["2823", "Pedernais (Bairro Girassol) - Pontinha (Metro)"]],
        223: [["2813", "Casal Novo - Lisboa (C. Militar)"], ["2814", "Casal Novo - Pontinha (Metro)"]],
        224: [["1711", "CaneÃ§as (Esc. SecundÃ¡ria) - Pontinha (Metro)"]],
        225: [["2216", "Odivelas (Metro) - Ramada (R. Heliodoro Salgado)"], ["2524", "Odivelas (Metro) | Circular, via Hospital Beatriz Ã‚ngelo"]],
        226: [["2202", "Arroja - Sr. Roubado (Metro)"], ["2810", "Arroja - Lisboa (C. Grande)"]],
        227: [["2701", "Pontinha (Metro) - Vale Grande"]],
        228: [["2781", "Loures (C.C. Continente) - Pontinha (Metro), via Ramada"], ["2815", "Jardim da Amoreira - Pontinha (Metro)"], ["2818", "Lisboa (C. Militar) - Serra da Amoreira"]],
        229: [["2218", "Odivelas (Metro) | Circular, via Colinas do Cruzeiro"], ["2758", "Mafra - Lisboa (C. Grande)"]],
        230: [["2631", "Casal de Cambra (C. SaÃºde) - Odivelas (Metro)"], ["2632", "Casal de Cambra (C. SaÃºde) - Odivelas (Metro), via Bairro Sol Nascente"]],
        231: [["1709", "CaneÃ§as - Lisboa (C. Militar)"]],
        233: [["2850", "Costa da Caparica - PÃ³voa de Santo AdriÃ£o (Parque Urbano)"]],
        235: [["2214", "Odivelas (Metro) - PÃ³voa de Santo AdriÃ£o (Parque Urbano)"], ["2221", "PÃ³voa de Santo AdriÃ£o (Parque Urbano) - Sr. Roubado (Metro)"], ["2516", "Casal Paradela - Sr. Roubado (Metro)"]],
        236: [["2832", "Casal Novo - Lisboa (C. Militar), via Casal de Cambra"], ["2833", "Pontinha (Metro) | Circular, via Casal Novo"]],
        237: [["2212", "Odivelas (C. Comercial) | Circular"], ["2754", "Lisboa (C. Grande) - PÃ³voa da Galega"], ["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        238: [["2518", "IKEA Loures - Sr. Roubado (Metro)"]],
        239: [["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        240: [["2220", "Olival Basto | Circular, via PÃ³voa de Santo AdriÃ£o e Odivelas (Metro)"]],
        241: [["2222", "Ramada | Circular"]],
        331: [["2764", "Bucelas - Lisboa (C. Grande), via S. JulÃ£o do Tojal"], ["2767", "Casainhos - Lisboa (C. Grande), via Lumiar"]],
        335: [["2510", "Bucelas - Sr. Roubado (Metro)"], ["2514", "Casainhos - Sr. Roubado (Metro), via Loures (C.Comercial)"], ["2750", "Bucelas - Lisboa (C. Grande), via CabeÃ§o de Montachique e Odivelas (Metro)"], ["2766", "Casainhos - Lisboa (C. Grande)"], ["2767", "Casainhos - Lisboa (C. Grande), via Lumiar"], ["2779", "Lisboa (C. Grande) - Santo AntÃ£o do Tojal, via Infantado e Loures (C.Comercial)"]],
        336: [["2762", "Bucelas - Lisboa (C. Grande), via A8"], ["2763", "Bucelas - Lisboa (C. Grande), via Lumiar"], ["2767", "Casainhos - Lisboa (C. Grande), via Lumiar"]],
        337: [["2525", "Santo AntÃ£o do Tojal - Sr. Roubado (Metro)"], ["2778", "Lisboa (C. Grande) - Santo AntÃ£o do Tojal"]],
        365: [["2520", "Loures (C.Comercial) - Sr. Roubado (Metro)"]],
        4: [["2209", "Casal Bispo - Sr. Roubado (Metro)"]],
        412: [["2769", "Cidade Nova - Lisboa (C. Grande)"]],
        413: [["2770", "Cidade Nova - Lisboa (C. Grande), via Lumiar"]],
        414: [["2771", "Cidade Nova - Lisboa (C. Grande), via Urb. Flores"]],
        415: [["2771", "Cidade Nova - Lisboa (C. Grande), via Urb. Flores"]],
        422: [["2772", "Lisboa (C. Grande) - Torres da Bela Vista"]],
        423: [["2773", "Lisboa (C. Grande) - Torres da Bela Vista, via Lumiar"]],
        424: [["2772", "Lisboa (C. Grande) - Torres da Bela Vista"]],
        425: [["2773", "Lisboa (C. Grande) - Torres da Bela Vista, via Lumiar"]],
        426: [["2772", "Lisboa (C. Grande) - Torres da Bela Vista"]],
        427: [["2773", "Lisboa (C. Grande) - Torres da Bela Vista, via Lumiar"]],
        450: [["2774", "Frielas - Lisboa (C. Grande)"]],
        451: [["2775", "Frielas - Lisboa (C. Grande), via Zona Industrial"]],
        470: [["2650", "Cidade Nova - Costa da Caparica"]],
        5: [["2630", "CaneÃ§as (Esc. SecundÃ¡ria) | Circular"]],
        702: [["2752", "Lisboa (C. Grande) - Malveira"]],
        810: [["2776", "Guerreiros - Lisboa (C. Grande)"]],
        811: [["2776", "Guerreiros - Lisboa (C. Grande)"]],
        812: [["2777", "Guerreiros - Lisboa (C. Grande), via Lumiar"]],
        813: [["2785", "Covas de Ferro - Lisboa (C. Grande)"]],
        814: [["2760", "Ã€-dos-Moninhos - Lisboa (C. Grande)"], ["2761", "Ã€-dos-Moninhos - Lisboa (C. Grande), via Bolores"]],
        815: [["2760", "Ã€-dos-Moninhos - Lisboa (C. Grande)"]],
        818: [["2785", "Covas de Ferro - Lisboa (C. Grande)"]],
        901: [["2207", "CaneÃ§as (Esc. SecundÃ¡ria) - Sr. Roubado (Metro), via Odivelas (Centro)"], ["2812", "CaneÃ§as (Esc. SecundÃ¡ria) - Lisboa (C. Grande), via Sr. Roubado (Metro)"]],
        905: [["2817", "Lisboa (C. Militar) - Odivelas (Metro)"], ["2822", "Odivelas (Metro) - Pontinha (Metro), via Serra da Luz"]],
        913: [["2205", "CaneÃ§as (Esc. SecundÃ¡ria) - Odivelas (Metro)"]],
        916: [["2213", "Odivelas (Metro) - FamÃµes | Circular"]],
        925: [["2211", "Jardim da Amoreira - Odivelas (Metro)"], ["2517", "Hospital Beatriz Ã‚ngelo - Odivelas (Metro) | Circular"]],
        926: [["2201", "Arroja - Odivelas (Metro)"]],
        931: [["2702", "Lisboa (C. Grande) - Pontinha (Metro)"]],
        934: [["2215", "Odivelas (Metro) - Ramada (Bairro Bons Dias)"], ["2523", "Odivelas (Metro) - Montemor"]],
        Nova: [["2204", "CaneÃ§as - FamÃµes"], ["2210", "Jardim da Amoreira - Odivelas"], ["2511", "Bairro dos CTT - Loures (C. Comercial)"], ["2512", "Bucelas - Sr. Roubado (Metro), via Ramada"], ["2610", "Odivelas (Metro) - UBBO"], ["2611", "UBBO - Ramada"]]
    },
    Oeiras: {
        1: [["1101", "Alfragide (Alegro) - AlgÃ©s (Terminal)"], ["1504", "AlgÃ©s (Terminal) - Bairro Zambujal, via Linda-a-Velha"], ["1722", "Alfragide (Alegro) - Hospital SÃ£o Francisco Xavier"]],
        10: [["1503", "AlgÃ©s (Terminal) - Bairro Zambujal"], ["1505", "AlgÃ©s (Terminal) - IKEA Alfragide"]],
        101: [["1717", "Lisboa (C. Militar) - Tercena, via Amadora Este (Metro)"]],
        102: [["1527", "Cruz Quebrada (EstaÃ§Ã£o) - Queluz (EstaÃ§Ã£o)"]],
        106: [["1530", "Oeiras (EstaÃ§Ã£o) - Queluz"], ["1601", "Amadora Este (Metro) - Carcavelos (Praia)"], ["1602", "Carcavelos (Praia) - Queluz"]],
        107: [["1716", "Idanha - Lisboa (M. Pombal)"]],
        108: [["1112", "Caxias - Queijas"], ["1507", "Caxias - Reboleira (Metro)"]],
        11: [["1724", "Linda-a-Velha - Lisboa (M. Pombal)"]],
        111: [["1120", "Oeiras (EstaÃ§Ã£o) - PaÃ§o de Arcos (EstaÃ§Ã£o)"]],
        112: [["1523", "CacÃ©m (EstaÃ§Ã£o) - Oeiras (EstaÃ§Ã£o)"]],
        113: [["1714", "Amadora (EstaÃ§Ã£o Sul) - BelÃ©m (EstaÃ§Ã£o)"]],
        114: [["1502", "AlgÃ©s (Terminal) - Amadora (EstaÃ§Ã£o Sul), via Linda-a-Velha"]],
        115: [["1725", "Lisboa (M. Pombal) - Oeiras (EstaÃ§Ã£o)"]],
        116: [["1124", "PaÃ§o de Arcos | Circular"]],
        117: [["1525", "Caxias - Monte AbraÃ£o (EstaÃ§Ã£o)"], ["1526", "Caxias (Est. Prisional) - Monte AbraÃ£o (EstaÃ§Ã£o)"]],
        119: [["1119", "Leceia - PaÃ§o de Arcos (EstaÃ§Ã£o)"], ["1531", "PaÃ§o de Arcos (EstaÃ§Ã£o) - SÃ£o Marcos"], ["1617", "PaÃ§o de Arcos (EstaÃ§Ã£o) - TalaÃ­de (Igreja)"]],
        12: [["1105", "AlgÃ©s (Terminal) - Queluz Baixo"], ["1108", "Carnaxide (Escola) - Queluz Baixo"], ["1522", "AlgÃ©s (Terminal) - Monte AbraÃ£o (EstaÃ§Ã£o)"]],
        122: [["1529", "Oeiras (EstaÃ§Ã£o) - Bairros dos Navegadores"]],
        125: [["1610", "PaÃ§o de Arcos (EstaÃ§Ã£o) - TalaÃ­de (Campo de Futebol)"], ["1616", "PaÃ§o de Arcos (EstaÃ§Ã£o) - Taguspark"]],
        129: [["1123", "PaÃ§o de Arcos (EstaÃ§Ã£o) - Porto Salvo (Lagoas Park)"]],
        13: [["1726", "Lisboa (M. Pombal) - Queijas"], ["1728", "Lisboa (M. Pombal) - Queijas, via Linda-a-Velha"], ["1730", "Lisboa (M. Pombal) - Queluz Baixo, via Linda-a-Velha"]],
        "13D": [["1727", "Lisboa (M. Pombal) - Queijas, via A5"], ["1729", "Lisboa (M. Pombal) - Queluz Baixo (C.C.)"]],
        149: [["1715", "BelÃ©m (EstaÃ§Ã£o) - Mira Sintra (Mercado)"]],
        15: [["1727", "Lisboa (M. Pombal) - Queijas, via A5"], ["1732", "Lisboa (M. Pombal) - SÃ£o Marcos, via Carnaxide"], ["1733", "Lisboa (M. Pombal) - SÃ£o Marcos, via Linda-a-Pastora"]],
        158: [["1113", "Caxias (EstaÃ§Ã£o) - Lage"], ["1114", "Caxias (EstaÃ§Ã£o) - PaÃ§o de Arcos (EstaÃ§Ã£o)"], ["1115", "Caxias (Pedreira Italiana) - Lage"], ["1116", "Caxias (Pedreira Italiana) - PaÃ§o de Arcos (EstaÃ§Ã£o)"], ["1117", "Caxias (Quinta da Moura) - Lage"], ["1118", "Caxias (Quinta da Moura) - PaÃ§o de Arcos (EstaÃ§Ã£o)"]],
        162: [["1713", "AlgÃ©s (Terminal) - Amadora Este (Metro)"]],
        171: [["1528", "Miraflores (Esc. SecundÃ¡ria) - Queluz (EstaÃ§Ã£o)"]],
        184: [["1611", "PaÃ§o de Arcos (EstaÃ§Ã£o) - TalaÃ­de (Campo de Futebol), via Vila Fria"]],
        2: [["1103", "AlgÃ©s (Terminal) - Queijas"], ["1107", "AlgÃ©s (Terminal) - Queluz Baixo, via Queijas"]],
        20: [["1712", "AlgÃ©s (Terminal) - Amadora (EstaÃ§Ã£o Sul)"]],
        23: [["1524", "Casal do CotÃ£o | Circular, via Tercena, SÃ£o Marcos e Taguspark"]],
        463: [["1613", "CacÃ©m (EstaÃ§Ã£o) - Oeiras (EstaÃ§Ã£o), via Trajouce"]],
        467: [["1614", "Carcavelos (EstaÃ§Ã£o) - Portela de Sintra (EstaÃ§Ã£o)"]],
        468: [["1615", "Carcavelos (EstaÃ§Ã£o) - Rio de Mouro (EstaÃ§Ã£o)"]],
        470: [["1607", "ConceiÃ§Ã£o da AbÃ³boda - Oeiras (EstaÃ§Ã£o)"], ["1608", "Oeiras (EstaÃ§Ã£o) - Taguspark"], ["1609", "Oeiras (EstaÃ§Ã£o) - TalaÃ­de (Igreja)"]],
        471: [["1604", "Carcavelos (EstaÃ§Ã£o) - Parede (Terminal)"]],
        479: [["1604", "Carcavelos (EstaÃ§Ã£o) - Parede (Terminal)"]],
        6: [["1104", "AlgÃ©s (Terminal) - Queijas, via Jamor"], ["1106", "AlgÃ©s (Terminal) - Queluz Baixo, via Jamor"]],
        "7/13": [["1723", "Carnaxide - Lisboa (M. Pombal)"]],
        "COMBUS Barcarena": [["1111", "Barcarena | Circular"]],
        "COMBUS Carnaxide": [["1109", "Carnaxide-Outurela"]],
        "COMBUS Cruz Quebrada": [["1102", "AlgÃ©s - Cruz Quebrada"]],
        "COMBUS Oeiras": [["1121", "Oeiras | Circular"]],
        "COMBUS PaÃ§o de Arcos": [["1122", "PaÃ§o de Arcos - Caxias"]],
        "COMBUS Porto Salvo": [["1125", "Porto Salvo | Circular"]],
        "COMBUS Queijas": [["1110", "Carnaxide-Queijas"]],
        Nova: [["1501", "Alfragide - Reboleira (EstaÃ§Ã£o) | Circular"], ["1506", "Amadora Hospital | Circular, via Alfragide"], ["1520", "AlgÃ©s (Terminal) - CacÃ©m (EstaÃ§Ã£o)"], ["1521", "AlgÃ©s (Terminal) - CacÃ©m (EstaÃ§Ã£o), via A5"], ["1605", "Carnaxide (Av. JoÃ£o Paulo II) - Nova SBE"], ["1606", "Carnaxide (Av. JoÃ£o Paulo II) - Nova SBE, via Terrugem"], ["1612", "CacÃ©m (EstaÃ§Ã£o) - Carcavelos (EstaÃ§Ã£o)"], ["1731", "CacÃ©m (EstaÃ§Ã£o) - Hospital SÃ£o Francisco Xavier"]]
    },
    Palmela: {
        257: [["4560", "Cabanas - Vila Nogueira de AzeitÃ£o"], ["4561", "Cabanas - Vila Nogueira de AzeitÃ£o, via Quinta do PicÃ£o"]],
        305: [["4610", "Bairro dos Marinheiros - Barreiro (Terminal)"]],
        "311 (Adaptado)": [["4611", "Penalva - Moita (Esc. SecundÃ¡ria)"]],
        "313 (Adaptado)": [["4532", "Moita - Quatro Marcos"]],
        "326 (Adaptado)": [["4531", "Moita - Palmela (Terminal)"]],
        413: [["4512", "Alcochete (Freeport) - SetÃºbal (ITS), via Alto Estanqueiro"], ["4513", "Alcochete (Freeport) - Pinhal Novo"], ["4517", "Montijo (Terminal RodoviÃ¡rio) - SetÃºbal (ITS)"], ["4523", "Montijo (Terminal RodoviÃ¡rio) - Pinhal Novo"]],
        414: [["4514", "Canha - Montijo (Terminal RodoviÃ¡rio), via PegÃµes"]],
        416: [["4514", "Canha - Montijo (Terminal RodoviÃ¡rio), via PegÃµes"], ["4515", "Montijo (Terminal RodoviÃ¡rio) - PegÃµes"]],
        426: [["4516", "Montijo (Terminal RodoviÃ¡rio) - Rio Frio"]],
        444: [["4520", "Faias - PegÃµes"]],
        446: [["4521", "Faias - Pinhal Novo"], ["4522", "Faias - PoceirÃ£o"]],
        447: [["4305", "Brejos do Assa - Palmela (Terminal)"], ["4524", "Palmela (Terminal) - PegÃµes"]],
        448: [["4322", "Pinhal Novo - Rio Frio"]],
        449: [["4320", "Pinhal Novo | Circular"], ["4321", "Pinhal Novo - Qta do Anjo"]],
        451: [["4530", "Bairro Vila Morena - Pinhal Novo"]],
        454: [["4302", "Palmela (EstaÃ§Ã£o) - Palmela (Terminal)"], ["4307", "Loja Nova - Palmela (Terminal)"]],
        455: [["4311", "Asseiceira - PoceirÃ£o"]],
        563: [["4715", "Lisboa (Oriente) - SetÃºbal (ITS), via Pinhal Novo"]],
        565: [["4710", "Lisboa (Oriente) - Palmela (Terminal)"], ["4711", "Lisboa (Oriente) - Pinhal Novo"]],
        604: [["4551", "Palmela (USF) - SetÃºbal (Av. LuÃ­sa Todi)"]],
        610: [["4541", "Algeruz - SetÃºbal (Av. LuÃ­sa Todi)"]],
        680: [["4301", "Palmela (Centro) - Palmela (Terminal)"]],
        708: [["4545", "Biscainho - SetÃºbal (Bela Vista)"]],
        709: [["4901", "Landeira - SetÃºbal (ITS)"]],
        710: [["4310", "Ã�guas de Moura - PoceirÃ£o"]],
        711: [["4312", "PoceirÃ£o - Vale Abrunheira (X), via Fernando PÃ³"]],
        758: [["4548", "LagameÃ§as - SetÃºbal (ITS)"]],
        764: [["4540", "Ã�guas de Moura - SetÃºbal (ITS)"], ["4544", "Bairro MargaÃ§a - SetÃºbal (ITS)"]],
        765: [["4540", "Ã�guas de Moura - SetÃºbal (ITS)"], ["4544", "Bairro MargaÃ§a - SetÃºbal (ITS)"]],
        767: [["4547", "Cabanas - SetÃºbal (ITS)"], ["4549", "Palmela (Terminal) - SetÃºbal (ITS)"], ["4562", "SetÃºbal (ITS) - Vila Nogueira de AzeitÃ£o, via Palmela (EstaÃ§Ã£o)"]],
        768: [["4306", "Cabanas - Palmela (Terminal)"], ["4550", "Palmela (Terminal) - Vila Nogueira de AzeitÃ£o"], ["4562", "SetÃºbal (ITS) - Vila Nogueira de AzeitÃ£o, via Palmela (EstaÃ§Ã£o)"]],
        774: [["4546", "Biscainho - SetÃºbal (ITS)"]],
        779: [["4542", "Algeruz - SetÃºbal (ITS)"], ["4543", "Algeruz - SetÃºbal (ITS), via PoÃ§oilos"]],
        8080: [["4905", "Faias - Vendas Novas"]],
        8902: [["4902", "Landeira - PegÃµes"], ["4906", "SetÃºbal (ITS) - Vendas Novas, via Landeira"]],
        Nova: [["3630", "AzeitÃ£o - Penalva (EstaÃ§Ã£o)"], ["4303", "Palmela | Circular"], ["4304", "Palmela (Terminal) - Penalva"], ["4308", "Palmela (Terminal) - Pinhal Novo (EstaÃ§Ã£o)"], ["4313", "Cabanas - Penalva"], ["4612", "Bairro dos Marinheiros - Palmela (Terminal)"]]
    },
    Seixal: {
        108: [["3103", "Corroios (EstaÃ§Ã£o) - Paio Pires (Farinheiras)"]],
        110: [["3513", "Cacilhas (Terminal) - Santa Marta do Pinhal"]],
        112: [["3114", "Foros de Amora - Paio Pires (Quinta FlamÃ¢ncia)"]],
        114: [["3508", "Cacilhas (Terminal) - Paio Pires (Centro)"]],
        116: [["3102", "Aroeira - Paio Pires (Quinta FlamÃ¢ncia)"], ["3122", "Verdizela - Cruz de Pau"], ["3521", "Cruz de Pau - Fonta da Telha"], ["3523", "Fonte da Telha - Paio Pires (Quinta FlamÃ¢ncia), via Seixal (Terminal Fluvial) e Foros de Amora (EstaÃ§Ã£o)"]],
        120: [["3515", "Caparica (Pilotos) -  Corroios"]],
        121: [["3526", "Laranjeiro - Pinheirinho"]],
        "126 (Adaptado)": [["3507", "Cacilhas (Terminal) - Marisol"], ["3524", "Hospital Garcia de Orta - Marisol"]],
        137: [["3110", "Fogueteiro (EstaÃ§Ã£o) - Redondos"], ["3120", "Redondos - Seixal (Terminal Fluvial)"]],
        139: [["3519", "Costa da Caparica (Terminal) - Corroios (EstaÃ§Ã£o)"]],
        143: [["3518", "Corroios (EstaÃ§Ã£o) - Vale de Figueira"]],
        149: [["3512", "Cacilhas (Terminal) - Quinta Princesa"]],
        159: [["3716", "Lisboa (Sete Rios) - Marisol"]],
        "159 (adaptada)": [["3501", "Almada Forum - Marisol, via Sobreda"]],
        162: [["3717", "Lisboa (Sete Rios) - Quinta do Brasileiro"]],
        163: [["3520", "Costa da Caparica (Terminal) - Quinta do Brasileiro"]],
        169: [["3715", "Lisboa (M. Pombal) - Santa Marta do Pinhal"]],
        172: [["3522", "Fonte da Telha - Paio Pires (Centro)"]],
        "175 (adaptada)": [["3501", "Almada Forum - Marisol, via Sobreda"]],
        "181 (adaptada)": [["3525", "Hospital Garcia de Orta - Miratejo"]],
        184: [["3111", "Fogueteiro (EstaÃ§Ã£o) - Seixal (Terminal Fluvial)"]],
        191: [["3514", "Cacilhas (Terminal) - Vale de MilhaÃ§os"], ["3104", "Corroios (EstaÃ§Ã£o) - Vale de MilhaÃ§os"]],
        192: [["3511", "Cacilhas (Terminal) - Pinheirinho"]],
        195: [["3119", "Pinhal Conde Cunha - Seixal (Terminal Fluvial)"]],
        196: [["3510", "Cacilhas (Terminal) - Pilotos"]],
        197: [["3504", "Bairro Fundo Fomento - Quintinha"]],
        198: [["3502", "Almada Forum - Paio Pires (Centro)"], ["3527", "Monte da Caparica (FCT) - Paio Pires (Bairro Cucena)"], ["3528", "Monte da Caparica (FCT) - Paio Pires (Centro)"]],
        199: [["3509", "Cacilhas (Terminal) - Paio Pires (Centro), via Seixal (Terminal Fluvial) e Amora"]],
        "1A": [["3101", "Amora - Foros de Amora (EstaÃ§Ã£o)"]],
        "1C": [["3516", "Charneca da Caparica - Corroios (EstaÃ§Ã£o)"]],
        "1F": [["3112", "Fogueteiro (EstaÃ§Ã£o) - Seixal (Terminal Fluvial), via Paio Pires"]],
        "1N": [["3620", "Coina (EstaÃ§Ã£o) - Quinta do Conde"]],
        203: [["3536", "Cacilhas (Terminal) - Sesimbra (Terminal)"]],
        207: [["3721", "Lisboa (Sete Rios) - Sesimbra (Terminal)"]],
        208: [["3635", "Coina (EstaÃ§Ã£o) - Sesimbra (Terminal), via AzeitÃ£o e Sampaio"]],
        211: [["3549", "Quinta do Conde - Sesimbra (Terminal), via Sampaio e Marco do Grilo"]],
        219: [["3548", "Marco do Grilo - Quinta do Conde"]],
        236: [["3107", "Laranjeiras - Marco do Grilo"]],
        252: [["3720", "Lisboa (Sete Rios) - Quinta do Conde"]],
        254: [["3535", "Cacilhas (Terminal) - Quinta do Conde"], ["3546", "Cruz de Pau - Quinta do Conde"], ["3547", "Fogueteiro (EstaÃ§Ã£o) - Quinta do Conde"]],
        260: [["3545", "Corroios (EstaÃ§Ã£o) - Sesimbra (Terminal)"]],
        "2A": [["3108", "Fogueteiro - Foros de Amora (EstaÃ§Ã£o)"]],
        "2C (Adaptado)": [["3506", "Cacilhas (Terminal) - Corroios (EstaÃ§Ã£o), via Miratejo"]],
        "2F": [["3113", "Fogueteiro (EstaÃ§Ã£o) - Seixal (Terminal Fluvial), via Quinta do Cabral"]],
        "2N": [["3626", "Coina (EstaÃ§Ã£o) - Vila Fresca de AzeitÃ£o"]],
        "2ND": [["3543", "Coina (EstaÃ§Ã£o) - Quinta do Conde, via Estrada de Coina"]],
        302: [["4620", "Moita - Paio Pires"]],
        "3C": [["3517", "Chegadinho - Corroios (EstaÃ§Ã£o)"]],
        "3F": [["3105", "FernÃ£o Ferro - Fogueteiro (EstaÃ§Ã£o)"]],
        "4F": [["3109", "Fogueteiro (EstaÃ§Ã£o) - Parque Empresarial do Seixal"]],
        583: [["3610", "Cacilhas (Terminal) - SetÃºbal (ITS), via A2"]],
        754: [["4631", "Fogueteiro (EstaÃ§Ã£o) - SetÃºbal (ITS)"]],
        755: [["4630", "Corroios (EstaÃ§Ã£o) - SetÃºbal (ITS)"]],
        783: [["3605", "Cacilhas (Terminal) - SetÃºbal (ITS), via AzeitÃ£o"]],
        Nova: [["3106", "Coina (EstaÃ§Ã£o) - FernÃ£o Ferro"], ["3115", "Marisol - Foros de Amora (EstaÃ§Ã£o), via Corroios (EstaÃ§Ã£o)"], ["3116", "Marisol - Seixal (Terminal Fluvial), via Corroios (EstaÃ§Ã£o)"], ["3117", "Marisol (Valadares) - Foros de Amora (EstaÃ§Ã£o)"], ["3118", "Marisol (Valadares) - Seixal (Terminal Fluvial)"], ["3121", "Seixal | Circular"], ["3503", "Almada Forum - Vale de MilhaÃ§os"], ["3505", "Cacilhas (Terminal) - Corroios (EstaÃ§Ã£o)"], ["3540", "Alfarim - Coina (EstaÃ§Ã£o)"], ["3541", "Coina (EstaÃ§Ã£o) - FernÃ£o Ferro, via Casal do Sapo e Pinhal do General"], ["3542", "Coina (EstaÃ§Ã£o) - Praia do Meco"], ["3544", "Coina (EstaÃ§Ã£o) - Sesimbra (Terminal)"], ["3601", "Barreiro - Cova da Piedade (Metro)"], ["3615", "Barreiro - Seixal"], ["3625", "Barreiro - Sesimbra (Terminal)"], ["4621", "Moita - Seixal (Terminal Fluvial)"]]
    },
    Sesimbra: {
        "1N": [["3620", "Coina (EstaÃ§Ã£o) - Quinta do Conde"]],
        201: [["3203", "Azoia - Sesimbra (Terminal)"], ["3204", "Azoia - Sesimbra (Terminal), via Serra da Azoia"], ["3205", "Cabo Espichel - Sesimbra (Terminal)"]],
        203: [["3536", "Cacilhas (Terminal) - Sesimbra (Terminal)"]],
        204: [["3206", "Carrasqueira - Sesimbra (Terminal)"]],
        205: [["3205", "Cabo Espichel - Sesimbra (Terminal)"]],
        207: [["3721", "Lisboa (Sete Rios) - Sesimbra (Terminal)"]],
        208: [["3635", "Coina (EstaÃ§Ã£o) - Sesimbra (Terminal), via AzeitÃ£o e Sampaio"], ["3641", "Quinta do Conde - Sampaio, via AzeitÃ£o"]],
        210: [["3217", "Azoia - Sampaio"]],
        211: [["3549", "Quinta do Conde - Sesimbra (Terminal), via Sampaio e Marco do Grilo"]],
        219: [["3548", "Marco do Grilo - Quinta do Conde"]],
        221: [["3220", "Sesimbra | Circular"]],
        222: [["3201", "Aldeia do Meco - Sesimbra (Terminal), via Aiana"], ["3202", "Alfarim - Sesimbra (Terminal), via Aiana"], ["3209", "Fornos - Sesimbra (Terminal), via Aiana"]],
        223: [["3208", "Fetais - Sesimbra (Terminal)"]],
        225: [["3640", "Azoia - Vila Nogueira de AzeitÃ£o"]],
        227: [["3212", "MaÃ§Ã£ (Rua Macieira) - Sesimbra (Terminal)"], ["3216", "Alto das Vinhas - Sampaio"]],
        228: [["3218", "Sesimbra (Porto de Abrigo) - Sesimbra (Terminal)"]],
        229: [["3213", "Pinhal de Cima - Sesimbra (Terminal) | Circular"]],
        230: [["4642", "Sesimbra (Terminal) - SetÃºbal (Hospital)"]],
        231: [["3201", "Aldeia do Meco - Sesimbra (Terminal), via Aiana"]],
        234: [["3215", "Fornos - Sampaio"]],
        240: [["3210", "Lagoa de Albufeira - Sesimbra (Terminal)"], ["3211", "Lagoa de Albufeira - Sesimbra (Terminal), via Sampaio"]],
        243: [["3207", "Carrasqueira - Sesimbra (Terminal), via Valbom e Sampaio"], ["3221", "Valbom - Sesimbra (Terminal), via Sampaio"]],
        245: [["3650", "Moita - Sesimbra (Terminal)"]],
        247: [["3214", "Sampaio - Santana"]],
        252: [["3720", "Lisboa (Sete Rios) - Quinta do Conde"]],
        254: [["3535", "Cacilhas (Terminal) - Quinta do Conde"], ["3546", "Cruz de Pau - Quinta do Conde"], ["3547", "Fogueteiro (EstaÃ§Ã£o) - Quinta do Conde"]],
        259: [["4640", "Casais da Serra - Vila Nogueira de AzeitÃ£o"]],
        260: [["3545", "Corroios (EstaÃ§Ã£o) - Sesimbra (Terminal)"]],
        "2N": [["3626", "Coina (EstaÃ§Ã£o) - Vila Fresca de AzeitÃ£o"]],
        "2ND": [["3543", "Coina (EstaÃ§Ã£o) - Quinta do Conde, via Estrada de Coina"]],
        440: [["4643", "Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)"]],
        754: [["4631", "Fogueteiro (EstaÃ§Ã£o) - SetÃºbal (ITS)"]],
        755: [["4630", "Corroios (EstaÃ§Ã£o) - SetÃºbal (ITS)"]],
        770: [["4641", "Quinta do Conde - SetÃºbal (ITS)"]],
        783: [["3605", "Cacilhas (Terminal) - SetÃºbal (ITS), via AzeitÃ£o"]],
        Nova: [["3219", "Sesimbra (R. Palames) - Sesimbra (Terminal)"], ["3222", "Quinta do Conde | Circular"], ["3540", "Alfarim - Coina (EstaÃ§Ã£o)"], ["3541", "Coina (EstaÃ§Ã£o) - FernÃ£o Ferro, via Casal do Sapo e Pinhal do General"], ["3542", "Coina (EstaÃ§Ã£o) - Praia do Meco"], ["3544", "Coina (EstaÃ§Ã£o) - Sesimbra (Terminal)"], ["3625", "Barreiro - Sesimbra (Terminal)"], ["3630", "AzeitÃ£o - Penalva (EstaÃ§Ã£o)"]]
    },
    "SetÃºbal": {
        208: [["3635", "Coina (EstaÃ§Ã£o) - Sesimbra (Terminal), via AzeitÃ£o e Sampaio"], ["3641", "Quinta do Conde - Sampaio, via AzeitÃ£o"]],
        225: [["3640", "Azoia - Vila Nogueira de AzeitÃ£o"]],
        230: [["4642", "Sesimbra (Terminal) - SetÃºbal (Hospital)"]],
        257: [["4560", "Cabanas - Vila Nogueira de AzeitÃ£o"], ["4561", "Cabanas - Vila Nogueira de AzeitÃ£o, via Quinta do PicÃ£o"]],
        259: [["4640", "Casais da Serra - Vila Nogueira de AzeitÃ£o"]],
        "2N": [["3626", "Coina (EstaÃ§Ã£o) - Vila Fresca de AzeitÃ£o"]],
        413: [["4512", "Alcochete (Freeport) - SetÃºbal (ITS), via Alto Estanqueiro"], ["4517", "Montijo (Terminal RodoviÃ¡rio) - SetÃºbal (ITS)"]],
        440: [["4643", "Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)"]],
        561: [["4725", "Lisboa (Sete Rios) - SetÃºbal (ITS)"]],
        562: [["4720", "Lisboa (Oriente) - SetÃºbal (ITS)"]],
        563: [["4715", "Lisboa (Oriente) - SetÃºbal (ITS), via Pinhal Novo"]],
        583: [["3610", "Cacilhas (Terminal) - SetÃºbal (ITS), via A2"]],
        601: [["4406", "Manteigadas - SetÃºbal (Mercado)"], ["4408", "Manteigadas - SetÃºbal (Mercado), via Bela Vista"], ["4412", "Morgada - SetÃºbal (Mercado)"], ["4413", "Morgada - SetÃºbal (Mercado), via Bela Vista"], ["4436", "SetÃºbal (Mercado) - SetÃºbal (Av. Soeiro Pereira Gomes)"]],
        602: [["4416", "PoÃ§o Mouro - SetÃºbal (ITS)"], ["4417", "PoÃ§o Mouro - SetÃºbal (ITS), via Manteigadas"], ["4418", "SetÃºbal (Alegro) - SetÃºbal (Av. 5 Outubro)"], ["4420", "SetÃºbal (Alegro) - SetÃºbal (ITS)"]],
        604: [["4403", "Fonte da Talha - SetÃºbal (Av. LuÃ­sa Todi)"], ["4423", "Amoreiras â€“ SetÃºbal (Av. LuÃ­sa Todi)"], ["4551", "Palmela (USF) - SetÃºbal (Av. LuÃ­sa Todi)"]],
        605: [["4431", "SetÃºbal (ITS) - SetÃºbal (Quinta Varzinha)"]],
        607: [["4434", "SetÃºbal (Mercado 2 de Abril) - SetÃºbal (R. Timor)"]],
        608: [["4424", "SetÃºbal (Bairro Viso) - Manteigadas"], ["4425", "SetÃºbal (Escola Viso) - Mitrena"], ["4426", "SetÃºbal (Bairro Viso) - SetÃºbal (CHEsetÃºbal)"]],
        609: [["4421", "SetÃºbal (Bairro Camolas) - SetÃºbal (Casal Figueiras)"], ["4422", "SetÃºbal (Bairro Camolas) - SetÃºbal (Casal Figueiras), via Bairro do Viso"], ["4428", "SetÃºbal (Casal Figueiras) - Vale Ana Gomes"]],
        610: [["4438", "SetÃºbal (Monte Belo Norte) - SetÃºbal (Saboaria)"], ["4440", "SetÃºbal (Monte Belo Norte) - SetÃºbal (Saboaria), via Alegro"], ["4541", "Algeruz - SetÃºbal (Av. LuÃ­sa Todi)"]],
        612: [["4419", "Brejos Canes - SetÃºbal (Saboaria)"], ["4441", "SetÃºbal (Saboaria) - SetÃºbal (Vale Cobro)"]],
        614: [["4421", "SetÃºbal (Bairro Camolas) - SetÃºbal (Casal Figueiras)"], ["4422", "SetÃºbal (Bairro Camolas) - SetÃºbal (Casal Figueiras), via Bairro do Viso"], ["4433", "Alto Guerra - SetÃºbal (Casal Figueiras)"]],
        616: [["4407", "Manteigadas - SetÃºbal (Mercado), via Bairro da Carmona"]],
        708: [["4545", "Biscainho - SetÃºbal (Bela Vista)"]],
        709: [["4901", "Landeira - SetÃºbal (ITS)"]],
        723: [["4472", "Praia do Creiro - SetÃºbal (ITS)"], ["4476", "Praias ArrÃ¡bida | Circular"]],
        725: [["4474", "Figueirinha - SetÃºbal (Alegro)"]],
        726: [["4471", "Praia Albarquel | Circular"]],
        727: [["4470", "Brejos AzeitÃ£o - Praia do Creiro"]],
        751: [["4414", "OutÃ£o (Hospital) - SetÃºbal (ITS)"], ["4415", "OutÃ£o (Hospital) - SetÃºbal (ITS), via vale da Rasca"]],
        754: [["4631", "Fogueteiro (EstaÃ§Ã£o) - SetÃºbal (ITS)"]],
        755: [["4630", "Corroios (EstaÃ§Ã£o) - SetÃºbal (ITS)"]],
        756: [["4452", "Mitrena (Portucel) - SetÃºbal (ITS)"]],
        757: [["4453", "Mitrena (Portucel) - SetÃºbal (ITS), via Estrada GraÃ§a"]],
        758: [["4548", "LagameÃ§as - SetÃºbal (ITS)"]],
        764: [["4540", "Ã�guas de Moura - SetÃºbal (ITS)"], ["4544", "Bairro MargaÃ§a - SetÃºbal (ITS)"]],
        765: [["4540", "Ã�guas de Moura - SetÃºbal (ITS)"], ["4544", "Bairro MargaÃ§a - SetÃºbal (ITS)"]],
        766: [["4442", "Praias do Sado (EstaÃ§Ã£o) - SetÃºbal (Bela Vista)"]],
        767: [["4547", "Cabanas - SetÃºbal (ITS)"], ["4549", "Palmela (Terminal) - SetÃºbal (ITS)"], ["4562", "SetÃºbal (ITS) - Vila Nogueira de AzeitÃ£o, via Palmela (EstaÃ§Ã£o)"]],
        768: [["4550", "Palmela (Terminal) - Vila Nogueira de AzeitÃ£o"], ["4562", "SetÃºbal (ITS) - Vila Nogueira de AzeitÃ£o, via Palmela (EstaÃ§Ã£o)"]],
        770: [["4641", "Quinta do Conde - SetÃºbal (ITS)"]],
        774: [["4546", "Biscainho - SetÃºbal (ITS)"]],
        776: [["4451", "Mitrena (Lisnave) - SetÃºbal (ITS)"]],
        779: [["4542", "Algeruz - SetÃºbal (ITS)"], ["4543", "Algeruz - SetÃºbal (ITS), via PoÃ§oilos"]],
        780: [["4402", "Estefanilha - SetÃºbal (ITS)"], ["4437", "FaralhÃ£o - SetÃºbal (ITS)"]],
        781: [["4411", "Morgada - SetÃºbal (ITS)"]],
        783: [["3605", "Cacilhas (Terminal) - SetÃºbal (ITS), via AzeitÃ£o"]],
        797: [["4439", "Praias do Sado - SetÃºbal (ITS)"]],
        8902: [["4906", "SetÃºbal (ITS) - Vendas Novas, via Landeira"]],
        Nova: [["3625", "Barreiro - Sesimbra (Terminal)"], ["3630", "AzeitÃ£o - Penalva (EstaÃ§Ã£o)"], ["4401", "Cachofarra - SetÃºbal (Hospital)"], ["4404", "Interfaces SetÃºbal | Circular"], ["4405", "Livramento-Montebelo | Circular"], ["4409", "Manteigadas - Viso"], ["4410", "Manteigadas (Esc. Profissional) - SetÃºbal (Alegro)"], ["4427", "SetÃºbal (Bela Vista) - SetÃºbal (Mercado)"], ["4429", "SetÃºbal (Centro SaÃºde) - SetÃºbal (Mercado)"], ["4430", "SetÃºbal (Hospital) - SetÃºbal (MontalvÃ£o)"], ["4432", "SetÃºbal (ITS) - Vale de Choupo"], ["4435", "Biscainho - FaralhÃ£o"], ["4443", "SetÃºbal (PolitÃ©cnico) - Praias do Sado"], ["4460", "AzeitÃ£o | Circular"], ["4475", "Portinho da ArrÃ¡bida - Viso"]]
    },
    Sintra: {
        101: [["1717", "Lisboa (C. Militar) - Tercena, via Amadora Este (Metro)"]],
        102: [["1527", "Cruz Quebrada (EstaÃ§Ã£o) - Queluz (EstaÃ§Ã£o)"]],
        103: [["1512", "Amadora (Hospital) - Montelavar"]],
        104: [["1508", "Almargem do Bispo - Amadora Este (Metro)"]],
        105: [["1518", "Monte AbraÃ£o - Reboleira (Metro)"]],
        106: [["1530", "Oeiras (EstaÃ§Ã£o) - Queluz"], ["1601", "Amadora Este (Metro) - Carcavelos (Praia)"], ["1602", "Carcavelos (Praia) - Queluz"]],
        107: [["1716", "Idanha - Lisboa (M. Pombal)"]],
        110: [["1229", "UrbanizaÃ§Ã£o Quinta das Flores (Junta de Freguesia) | Circular"]],
        112: [["1211", "Belas - CacÃ©m (EstaÃ§Ã£o)"], ["1523", "CacÃ©m (EstaÃ§Ã£o) - Oeiras (EstaÃ§Ã£o)"]],
        117: [["1525", "Caxias - Monte AbraÃ£o (EstaÃ§Ã£o)"], ["1526", "Caxias (Est. Prisional) - Monte AbraÃ£o (EstaÃ§Ã£o)"]],
        119: [["1531", "PaÃ§o de Arcos (EstaÃ§Ã£o) - SÃ£o Marcos"], ["1617", "PaÃ§o de Arcos (EstaÃ§Ã£o) - TalaÃ­de (Igreja)"]],
        12: [["1522", "AlgÃ©s (Terminal) - Monte AbraÃ£o (EstaÃ§Ã£o)"]],
        122: [["1529", "Oeiras (EstaÃ§Ã£o) - Bairros dos Navegadores"]],
        124: [["1634", "Fonte da Aranha - Montelavar"]],
        125: [["1616", "PaÃ§o de Arcos (EstaÃ§Ã£o) - Taguspark"]],
        126: [["1221", "CacÃ©m (EstaÃ§Ã£o) | Circular"]],
        130: [["1230", "Monte AbraÃ£o (EstaÃ§Ã£o) | Circular"]],
        131: [["1230", "Monte AbraÃ£o (EstaÃ§Ã£o) | Circular"]],
        132: [["1515", "Amadora Este (Metro) - Casal de Cambra"], ["1720", "Casal de Cambra - Lisboa (C. Militar), via Amadora"]],
        133: [["1603", "Amadora (EstaÃ§Ã£o Norte) - CaneÃ§as"]],
        134: [["1517", "Casal de Cambra - Reboleira (Metro)"]],
        140: [["1220", "CacÃ©m (EstaÃ§Ã£o) - SÃ£o Marcos (Largo)"], ["1233", "Mira Sintra (Mercado) - SÃ£o Marcos (Largo)"]],
        144: [["1718", "CacÃ©m (Bairro Grajal) - BelÃ©m (EstaÃ§Ã£o)"]],
        149: [["1715", "BelÃ©m (EstaÃ§Ã£o) - Mira Sintra (Mercado)"]],
        "149/Nova": [["1511", "Amadora (Hospital) - Monte AbraÃ£o (EstaÃ§Ã£o)"]],
        15: [["1732", "Lisboa (M. Pombal) - SÃ£o Marcos, via Carnaxide"], ["1733", "Lisboa (M. Pombal) - SÃ£o Marcos, via Linda-a-Pastora"]],
        150: [["1213", "CacÃ©m (Bairro Joaquim Fontes - Shopping) - CacÃ©m (EstaÃ§Ã£o)"]],
        151: [["1218", "CacÃ©m (EstaÃ§Ã£o) - Mira Sintra (Mercado)"], ["1219", "CacÃ©m (EstaÃ§Ã£o) - Mira Sintra (Mercado), via Av. Santa Maria"]],
        152: [["1204", "Alegro Sintra | Circular"]],
        "154 (Adaptado)": [["1514", "Amadora (Hospital) | Circular, via Brandoa"]],
        "155 (Adaptado)": [["1514", "Amadora (Hospital) | Circular, via Brandoa"]],
        157: [["1519", "Queluz (PalÃ¡cio) - Serra da Silveira"]],
        160: [["1209", "Bairro da Felosa - Mira Sintra (Mercado)"]],
        161: [["1222", "CacÃ©m (EstaÃ§Ã£o) | Circular, via Bairro Joaquim Fontes"]],
        163: [["1227", "MassamÃ¡ (Casal do Olival) - Queluz"], ["1721", "Lisboa (C. Militar) - MassamÃ¡ (Casal do Olival), via Amadora Este (Metro)"]],
        164: [["1224", "Carregueira (Estabelecimento prisional) - Monte AbraÃ£o (EstaÃ§Ã£o)"]],
        170: [["1215", "CacÃ©m (EstaÃ§Ã£o) - Encosta de SÃ£o Marcos"], ["1217", "CacÃ©m (EstaÃ§Ã£o) - MassamÃ¡ (UrbanizaÃ§Ã£o Norte)"], ["1225", "Encosta de SÃ£o Marcos - MassamÃ¡ (UrbanizaÃ§Ã£o Norte)"]],
        171: [["1528", "Miraflores (Esc. SecundÃ¡ria) - Queluz (EstaÃ§Ã£o)"]],
        179: [["1212", "CacÃ©m (Bairro Grajal) - Queluz (EstaÃ§Ã£o)"]],
        "186 (Adaptado)": [["1510", "Amadora (Hospital) - Damaia (Praceta Liberdade)"]],
        202: [["1633", "Ericeira (Terminal) - Portela de Sintra (EstaÃ§Ã£o)"]],
        203: [["2830", "Casal Bispo - Lisboa (C. Militar)"], ["2831", "Casal Bispo - Pontinha (Metro)"]],
        210: [["1709", "CaneÃ§as - Lisboa (C. Militar)"], ["1710", "CaneÃ§as - Pontinha (Metro)"]],
        215: [["2605", "CacÃ©m (EstaÃ§Ã£o) - Loures (Lg Marcos RomÃ£o Reis JÃºnior)"], ["2606", "CacÃ©m (EstaÃ§Ã£o) - CaneÃ§as (Casa da Cultura)"]],
        22: [["1216", "Casal do CotÃ£o | Circular, via Tercena e SÃ£o Marcos"]],
        220: [["1636", "Bairro Arco Maria Teresa - Dona Maria"], ["1637", "Casal de Cambra - Vale de Lobos"], ["2626", "Mafra (Parque Desportivo) - Portela de Sintra (EstaÃ§Ã£o)"]],
        221: [["1635", "Almargem do Bispo (Centro de SaÃºde) - CaneÃ§as"]],
        224: [["1711", "CaneÃ§as (Esc. SecundÃ¡ria) - Pontinha (Metro)"]],
        23: [["1524", "Casal do CotÃ£o | Circular, via Tercena, SÃ£o Marcos e Taguspark"]],
        230: [["2631", "Casal de Cambra (C. SaÃºde) - Odivelas (Metro)"], ["2632", "Casal de Cambra (C. SaÃºde) - Odivelas (Metro), via Bairro Sol Nascente"]],
        231: [["1709", "CaneÃ§as - Lisboa (C. Militar)"]],
        236: [["2832", "Casal Novo - Lisboa (C. Militar), via Casal de Cambra"], ["2833", "Pontinha (Metro) | Circular, via Casal Novo"]],
        239: [["2627", "Mafra (Parque Desportivo) - Ribeira dos TostÃµes"]],
        24: [["1226", "MassamÃ¡ - Queluz (EstaÃ§Ã£o)"]],
        25: [["1513", "Amadora (Hospital) | Circular"]],
        253: [["1201", "Alcolombal de Cima - Terrugem (Escola), via SacÃ¡rio"]],
        254: [["1255", "Rebanque - Portela de Sintra (EstaÃ§Ã£o)"]],
        255: [["1240", "Negrais - Portela de Sintra (EstaÃ§Ã£o)"]],
        403: [["1624", "Cascais (Terminal) - Portela de Sintra (EstaÃ§Ã£o), via AzÃ³ia e AlmoÃ§ageme"]],
        "403 (Adaptado)": [["1253", "Circuito Cabo da Roca - Portela de Sintra"]],
        417: [["1623", "Cascais (Terminal) - Portela de Sintra (EstaÃ§Ã£o)"], ["1626", "CascaiShopping - Portela de Sintra (EstaÃ§Ã£o)"]],
        418: [["1621", "Bairro da Cruz Vermelha - Portela de Sintra (EstaÃ§Ã£o)"], ["1626", "CascaiShopping - Portela de Sintra (EstaÃ§Ã£o)"], ["1629", "Estoril (EstaÃ§Ã£o) - Portela de Sintra (EstaÃ§Ã£o)"], ["1630", "Estoril (EstaÃ§Ã£o) - Portela de Sintra (EstaÃ§Ã£o), via Monte Estoril e Amoreira"]],
        433: [["1252", "Portela de Sintra (EstaÃ§Ã£o) | Circular"]],
        437: [["1206", "AlgueirÃ£o-Mem Martins (EstaÃ§Ã£o) - Portela de Sintra (EstaÃ§Ã£o), via Tribunal"]],
        "439 (Adaptado)": [["1250", "Portela de Sintra (EstaÃ§Ã£o) - Praia Grande"]],
        "440 (Adaptado)": [["1247", "Fontanelas - Portela de Sintra (EstaÃ§Ã£o), via Janas"]],
        441: [["1241", "AlmoÃ§ageme (Av. Dr. B. Vasc. X R. Sto AndrÃ©) - Portela de Sintra (EstaÃ§Ã£o)"]],
        "441 (Adaptado)": [["1254", "Circuito Praias - Portela de Sintra"]],
        442: [["1246", "Fontanelas - Portela de Sintra (EstaÃ§Ã£o), via Casal da Granja"], ["1248", "Portela de Sintra (EstaÃ§Ã£o) - Praia das MaÃ§Ã£s (Piscinas)"]],
        443: [["1245", "Catribana - Portela de Sintra (EstaÃ§Ã£o)"]],
        444: [["1249", "Portela de Sintra (EstaÃ§Ã£o) - Praia do Magoito"]],
        445: [["1243", "Casais da Cabrela - Portela de Sintra (EstaÃ§Ã£o)"], ["1632", "Carvalhal - Portela de Sintra (EstaÃ§Ã£o)"]],
        446: [["1234", "ShopBus Mira Sintra"]],
        447: [["1208", "Almargem do Bispo (CemitÃ©rio) - Portela de Sintra (EstaÃ§Ã£o)"], ["1235", "Covas de Ferro - MercÃªs (EstaÃ§Ã£o)"]],
        "447 (adaptada)": [["1231", "Mem Martins (Esc. Ferreira Castro) - Coutinho Afonso"]],
        448: [["1214", "CacÃ©m (EstaÃ§Ã£o) - Alegro Sintra"], ["1223", "ShopBus CacÃ©m"]],
        450: [["1210", "Bairro Fitares - Mem Martins (Esc. SecundÃ¡ria)"]],
        455: [["1625", "Cascais (Terminal) - Rio de Mouro (EstaÃ§Ã£o)"]],
        456: [["1627", "CascaiShopping - Rio de Mouro (EstaÃ§Ã£o)"], ["1628", "Estoril (EstaÃ§Ã£o) -  Rio de Mouro (EstaÃ§Ã£o)"], ["1631", "Estoril (EstaÃ§Ã£o) - Rio de Mouro (EstaÃ§Ã£o) | Direta"]],
        458: [["1205", "AlgueirÃ£o-Mem Martins (EstaÃ§Ã£o) - Mem Martins (Esc. SecundÃ¡ria)"]],
        460: [["1202", "Alegro Sintra - Mem Martins | Circular"], ["1203", "Alegro Sintra - Rio de Mouro (EstaÃ§Ã£o)"], ["1207", "AlgueirÃ£o-Mem Martins (EstaÃ§Ã£o) - Rio de Mouro (EstaÃ§Ã£o)"], ["1251", "Portela de Sintra (EstaÃ§Ã£o) - Rio de Mouro (EstaÃ§Ã£o)"]],
        463: [["1613", "CacÃ©m (EstaÃ§Ã£o) - Oeiras (EstaÃ§Ã£o), via Trajouce"], ["1618", "AbÃ³boda (Auto BarÃ£o) - CacÃ©m (EstaÃ§Ã£o)"], ["1622", "CacÃ©m (EstaÃ§Ã£o) - Carcavelos (EstaÃ§Ã£o), via Trajouce"]],
        467: [["1614", "Carcavelos (EstaÃ§Ã£o) - Portela de Sintra (EstaÃ§Ã£o)"], ["1619", "AbÃ³boda (Auto BarÃ£o) - Portela de Sintra (EstaÃ§Ã£o)"]],
        468: [["1615", "Carcavelos (EstaÃ§Ã£o) - Rio de Mouro (EstaÃ§Ã£o)"], ["1620", "AbÃ³boda (Auto BarÃ£o) - Rio de Mouro (EstaÃ§Ã£o)"]],
        5: [["2630", "CaneÃ§as (Esc. SecundÃ¡ria) | Circular"]],
        813: [["2785", "Covas de Ferro - Lisboa (C. Grande)"]],
        816: [["2621", "Covas de Ferro - Pinheiro de Loures"]],
        818: [["2785", "Covas de Ferro - Lisboa (C. Grande)"]],
        822: [["2620", "Covas de Ferro - Loures (Centro SaÃºde), via A-Dos-Calvos"]],
        831: [["2620", "Covas de Ferro - Loures (Centro SaÃºde), via A-Dos-Calvos"]],
        931: [["2702", "Lisboa (C. Grande) - Pontinha (Metro)"]],
        Nova: [["1228", "MassamÃ¡ (EstaÃ§Ã£o) | Circular, via Belas"], ["1232", "Rio de Mouro (EstaÃ§Ã£o) | Circular, via Mem Martins"], ["1242", "Cabrela - VÃ¡rzea de Colares"], ["1244", "Casal de Cambra - Portela de Sintra (EstaÃ§Ã£o)"], ["1509", "Amadora (Hospital) - Casal de Cambra (C. SaÃºde)"], ["1516", "Casal de Cambra - Monte AbraÃ£o (EstaÃ§Ã£o)"], ["1520", "AlgÃ©s (Terminal) - CacÃ©m (EstaÃ§Ã£o)"], ["1521", "AlgÃ©s (Terminal) - CacÃ©m (EstaÃ§Ã£o), via A5"], ["1612", "CacÃ©m (EstaÃ§Ã£o) - Carcavelos (EstaÃ§Ã£o)"], ["1719", "Casal de Cambra - Lisboa (C. Militar)"], ["1731", "CacÃ©m (EstaÃ§Ã£o) - Hospital SÃ£o Francisco Xavier"], ["2625", "Mafra - SÃ£o JoÃ£o das Lampas"]]
    },
    "Sobral de Monte AgraÃ§o (CIM Oeste)": {
        224: [["2913", "Mafra (Parque Desportivo) - Pero Negro (EstaÃ§Ã£o)"]],
        230: [["2900", "Lisboa (C. Grande) - SÃ£o SebastiÃ£o"], ["2915", "Malveira - Vila Franca do RosÃ¡rio"]],
        231: [["2910", "Ervideira - Malveira"], ["2911", "Gradil - Malveira"], ["2912", "Gradil - Malveira (Terminal)"]],
        721: [["2913", "Mafra (Parque Desportivo) - Pero Negro (EstaÃ§Ã£o)"]]
    },
    "Torres Vedras (CIM Oeste)": {
        204: [["2905", "Aranha (Rotunda) - Ericeira (Terminal)"], ["2907", "Assenta - Ericeira (Terminal)"]],
        221: [["2914", "Mafra (Parque Desportivo) - SÃ£o Pedro da Cadeira (Rotunda da Aranha)"]],
        241: [["2906", "Assenta - EncarnaÃ§Ã£o"], ["2908", "Cambelas - EncarnaÃ§Ã£o"], ["2909", "Cambelas - Freiria (E.B. 2-3)"]]
    },
    "Vendas Novas (CIM Alentejo Central)": {
        709: [["4901", "Landeira - SetÃºbal (ITS)"]],
        8080: [["4905", "Faias - Vendas Novas"]],
        8902: [["4902", "Landeira - PegÃµes"], ["4906", "SetÃºbal (ITS) - Vendas Novas, via Landeira"]]
    },
    "Vila Franca de Xira": {
        14: [["2310", "Bom Retiro - Carregado (Atral-CIPAN)"], ["2313", "Carregado (Atral-CIPAN) - Vila Franca de Xira (EstaÃ§Ã£o)"], ["2314", "Castanheira do Ribatejo - Vila Franca de Xira (EstaÃ§Ã£o)"]],
        153: [["2303", "Alverca (EstaÃ§Ã£o) - Arcena"]],
        16: [["2928", "Casais da Granja - Vila Franca de Xira (EstaÃ§Ã£o)"]],
        18: [["2925", "Cadafais - Vila Franca de Xira (Terminal), via Bairro da Mata"], ["2926", "Cadafais - Vila Franca de Xira (Terminal), via Cachoeiras"]],
        19: [["2311", "Cachoeiras - Castanheira do Ribatejo"], ["2312", "Cachoeiras - Vila Franca de Xira (Terminal)"], ["2320", "Monte Gordo - Vila Franca de Xira (EstaÃ§Ã£o)"], ["2321", "Monte Gordo - Vila Franca de Xira (Terminal)"], ["2927", "Cadafais - Vila Franca de Xira (Terminal), via Loja Nova"]],
        3: [["2842", "Lisboa (C. Grande) - Vila Franca de Xira"]],
        319: [["2791", "Alverca (Z. Industrial) - Lisboa (C. Grande)"]],
        320: [["2790", "Alverca (EstaÃ§Ã£o) - Lisboa (C. Grande)"], ["2793", "Forte da Casa - Lisboa (C. Grande)"]],
        322: [["2319", "Forte da Casa | Circular"]],
        323: [["2318", "Forte da Casa - PÃ³voa de Santa Iria (EstaÃ§Ã£o) | Circular"]],
        324: [["2323", "PÃ³voa de Santa Iria | Circular"]],
        325: [["2536", "PÃ³voa de Santa Iria - SalvaÃ§Ã£o | Circular"]],
        326: [["2323", "PÃ³voa de Santa Iria | Circular"]],
        328: [["2323", "PÃ³voa de Santa Iria | Circular"]],
        329: [["2795", "Lisboa (C. Grande) - Quinta da Piedade"]],
        330: [["2794", "Forte da Casa - Lisboa (Oriente)"]],
        342: [["2530", "Alverca (EstaÃ§Ã£o) - Bucelas, via Malvarosa"]],
        345: [["2792", "Arcena - Lisboa (Oriente)"]],
        346: [["2531", "Alverca (EstaÃ§Ã£o) - Granja, via Casal das Areias"], ["2533", "Alverca (EstaÃ§Ã£o) - Santo AntÃ£o do Tojal, via Casal das Areias"]],
        347: [["2840", "Arcena - Lisboa (Oriente), via A1"]],
        348: [["2537", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Santo AntÃ£o do Tojal"]],
        349: [["2317", "Fonte Santa - PÃ³voa de Santa Iria (EstaÃ§Ã£o), via Morgado"], ["2326", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Vialonga"]],
        350: [["2325", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Verdelho Ruivo"]],
        351: [["2327", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Vialonga | Circular"]],
        353: [["2797", "Lisboa (C. Grande) - Vialonga, via A8"], ["2798", "Lisboa (C. Grande) - Vialonga, via A9 e A8"]],
        354: [["2796", "Lisboa (C. Grande) - Vialonga"]],
        355: [["2651", "Bucelas - Costa da Caparica (Terminal)"]],
        360: [["2532", "Alverca (EstaÃ§Ã£o) - Loures"]],
        370: [["2537", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Santo AntÃ£o do Tojal"], ["2538", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Santo AntÃ£o do Tojal, via Zambujal"]],
        39: [["2307", "Arcena - Vila Franca de Xira (Terminal)"]],
        40: [["2302", "Alverca - Vila Franca de Xira (Terminal)"], ["2324", "PÃ³voa de Santa Iria - Vila Franca de Xira (Terminal)"], ["2328", "PÃ³voa de Santa Iria (Qta Piedade) - Vila Franca de Xira (Terminal)"]],
        41: [["2324", "PÃ³voa de Santa Iria - Vila Franca de Xira (Terminal)"]],
        44: [["2306", "Alverca (EstaÃ§Ã£o) - Vila Franca de Xira (Terminal)"]],
        45: [["2329", "Qta da Cruz de Pau - Sobralinho"]],
        48: [["2302", "Alverca - Vila Franca de Xira (Terminal)"], ["2842", "Lisboa (C. Grande) - Vila Franca de Xira"]],
        49: [["2331", "Torre de Cima e Capelas - Vila Franca de Xira (Hospital), via Povos"], ["2332", "Vila Franca de Xira (EstaÃ§Ã£o) - Vila Franca de Xira (Hospital)"], ["2333", "Vila Franca de Xira (EstaÃ§Ã£o) - Vila Franca de Xira (Hospital), via N1"]],
        50: [["2316", "Cotovios - Vila Franca de Xira (Terminal)"]],
        53: [["2301", "Alhandra - BogalhÃ£o"], ["2308", "BogalhÃ£o - Vila Franca de Xira (Terminal)"], ["2309", "BogalhÃ£o - Vila Franca de Xira (Terminal), via Subserra"], ["2330", "Subserra - Vila Franca de Xira (Terminal)"]],
        57: [["2304", "Alverca (EstaÃ§Ã£o) - Bulhaco"]],
        58: [["2920", "A-do-MourÃ£o - Alverca (EstaÃ§Ã£o)"]],
        59: [["2921", "Alhandra - Arruda dos Vinhos"], ["2922", "Arruda dos Vinhos - Bulhaco"]],
        701: [["2652", "Costa da Caparica (Terminal) - Forte da Casa"]],
        72: [["2841", "Lisboa (C. Grande) - Sobralinho"]],
        91: [["2332", "Vila Franca de Xira (EstaÃ§Ã£o) - Vila Franca de Xira (Hospital)"]],
        Nova: [["2305", "Alverca (EstaÃ§Ã£o) - Malvarosa | Circular"], ["2315", "Castanheira do Ribatejo | Circular"], ["2322", "Bolonha - PÃ³voa de Santa Iria (EstaÃ§Ã£o)"], ["2534", "Loures - Santa Iria da AzÃ³ia"], ["2535", "Loures - Vila Franca de Xira"], ["2539", "SacavÃ©m (EstaÃ§Ã£o) - Santa Iria da AzÃ³ia (EstaÃ§Ã£o), via Portela da AzÃ³ia"], ["2540", "Santa Iria da AzÃ³ia - Vialonga"]]
    }
}, diretorio_operadores = {
    "Barraqueiro Transportes": {
        14: [["2310", "Bom Retiro - Carregado (Atral-CIPAN)"], ["2313", "Carregado (Atral-CIPAN) - Vila Franca de Xira (EstaÃ§Ã£o)"], ["2314", "Castanheira do Ribatejo - Vila Franca de Xira (EstaÃ§Ã£o)"]],
        16: [["2928", "Casais da Granja - Vila Franca de Xira (EstaÃ§Ã£o)"]],
        18: [["2925", "Cadafais - Vila Franca de Xira (Terminal), via Bairro da Mata"], ["2926", "Cadafais - Vila Franca de Xira (Terminal), via Cachoeiras"]],
        19: [["2311", "Cachoeiras - Castanheira do Ribatejo"], ["2312", "Cachoeiras - Vila Franca de Xira (Terminal)"], ["2320", "Monte Gordo - Vila Franca de Xira (EstaÃ§Ã£o)"], ["2321", "Monte Gordo - Vila Franca de Xira (Terminal)"], ["2927", "Cadafais - Vila Franca de Xira (Terminal), via Loja Nova"]],
        200: [["2740", "Ericeira (Terminal) - Lisboa (C. Grande), via A8"], ["2741", "Ericeira (Terminal) - Lisboa (C. Grande), via Ericeira (Centro), Freixeira e A8"], ["2804", "Mafra - Lisboa (C. Grande), via A8"]],
        202: [["1633", "Ericeira (Terminal) - Portela de Sintra (EstaÃ§Ã£o)"]],
        203: [["2128", "Ericeira (Terminal) - Fonte Boa da Brincosa"], ["2133", "Ericeira (Terminal) - Mafra (PalÃ¡cio), via Carvoeira, Montesouros e Av. Portugal"], ["2143", "Mafra (PalÃ¡cio) - Zambujal"]],
        204: [["2112", "Casais de SÃ£o LourenÃ§o - Ericeira (Terminal)"], ["2127", "Ericeira (Terminal) - Barril"], ["2905", "Aranha (Rotunda) - Ericeira (Terminal)"], ["2907", "Assenta - Ericeira (Terminal)"]],
        205: [["2118", "EncarnaÃ§Ã£o (CemitÃ©rio) - Ericeira (Terminal), via Santo Isidoro e Monte Godel"]],
        206: [["2113", "Casais de SÃ£o LourenÃ§o - Ericeira (Terminal), via Feiteira"], ["2117", "EncarnaÃ§Ã£o (CemitÃ©rio) - Ericeira (Terminal), via Ribamar"]],
        207: [["2801", "Ericeira (Terminal) - Lisboa (C. Grande), via A21/A8"]],
        208: [["2132", "Ericeira (Terminal) - Mafra"], ["2740", "Ericeira (Terminal) - Lisboa (C. Grande), via A8"], ["2741", "Ericeira (Terminal) - Lisboa (C. Grande), via Ericeira (Centro), Freixeira e A8"], ["2742", "Lisboa (C. Grande) - Mafra (Terminal)"], ["2751", "Ericeira (Terminal) - Lisboa (C. Grande)"], ["2758", "Mafra - Lisboa (C. Grande)"]],
        209: [["2802", "Lisboa (C. Grande) - Mafra (Terminal), via A21"], ["2803", "Ericeira (Terminal) - Lisboa (C. Grande), via Mafra e A21/A8"]],
        210: [["2125", "Urbana da Ericeira 2"]],
        211: [["2124", "Urbana da Ericeira 1"]],
        215: [["2102", "A-da-Perra - Lagoa (ColÃ©gio Miramar)"], ["2137", "Lagoa (ColÃ©gio Miramar) - Mafra (PalÃ¡cio)"], ["2140", "Lagoa (ColÃ©gio Miramar) - Sobreiro"]],
        216: [["2101", "Achada - Lagoa (ColÃ©gio Miramar)"], ["2129", "Ericeira (Terminal) - Lagoa (ColÃ©gio Miramar)"], ["2130", "Ericeira (Terminal) - Lagoa (ColÃ©gio Miramar), via Ribamar"], ["2131", "Ericeira (Terminal) - Lagoa (ColÃ©gio Miramar), via Santo Isidoro"]],
        217: [["2111", "Casais da Areia - Lagoa (ColÃ©gio Miramar)"], ["2114", "Charneca - Lagoa (ColÃ©gio Miramar)"], ["2119", "EncarnaÃ§Ã£o (CemitÃ©rio) - Lagoa (ColÃ©gio Miramar)"]],
        218: [["2138", "Lagoa (ColÃ©gio Miramar) - SÃ£o Domingos"]],
        219: [["2115", "CodeÃ§al (Tapada Nacional) - Lagoa (ColÃ©gio Miramar)"], ["2139", "Lagoa (ColÃ©gio Miramar) - Sobral da Abelheira"]],
        220: [["2626", "Mafra (Parque Desportivo) - Portela de Sintra (EstaÃ§Ã£o)"]],
        221: [["2105", "Barreiralva (Igreja) - Mafra (Parque Desportivo)"], ["2116", "EncarnaÃ§Ã£o - Mafra (Parque Desportivo)"], ["2914", "Mafra (Parque Desportivo) - SÃ£o Pedro da Cadeira (Rotunda da Aranha)"]],
        222: [["2103", "Antas - Mafra (Parque Desportivo)"]],
        223: [["2144", "Mafra (Parque Desportivo) - Livramento"]],
        224: [["2134", "Ervideira - Gradil"], ["2913", "Mafra (Parque Desportivo) - Pero Negro (EstaÃ§Ã£o)"]],
        225: [["2116", "EncarnaÃ§Ã£o - Mafra (Parque Desportivo)"], ["2805", "EncarnaÃ§Ã£o - Lisboa (C. Grande)"]],
        226: [["2104", "Avessada - Malveira"], ["2106", "Bocal - Mafra (Parque Desportivo)"], ["2107", "Bocal - Malveira, via Avessada e Portela"]],
        227: [["2110", "Carvalhal - Mafra (Parque Desportivo), via Mata Grande e Valverde"]],
        229: [["2758", "Mafra - Lisboa (C. Grande)"], ["2807", "Lisboa (C. Grande) - Zambujal, via Mafra"]],
        230: [["2900", "Lisboa (C. Grande) - SÃ£o SebastiÃ£o"], ["2915", "Malveira - Vila Franca do RosÃ¡rio"]],
        231: [["2910", "Ervideira - Malveira"], ["2911", "Gradil - Malveira"], ["2912", "Gradil - Malveira (Terminal)"]],
        233: [["2145", "Mafra | Circular, via Sobral da Abelheira"]],
        238: [["2150", "Valverde - Venda do Pinheiro"]],
        239: [["2135", "Igreja Nova - Venda do Pinheiro (NÃºcleo Empresarial sul)"], ["2627", "Mafra (Parque Desportivo) - Ribeira dos TostÃµes"]],
        241: [["2906", "Assenta - EncarnaÃ§Ã£o"], ["2908", "Cambelas - EncarnaÃ§Ã£o"], ["2909", "Cambelas - Freiria (E.B. 2-3)"]],
        246: [["2801", "Ericeira (Terminal) - Lisboa (C. Grande), via A21/A8"]],
        248: [["2123", "Praia da Foz do Lizandro - Praia de Ribeira d'Ilhas"]],
        252: [["2142", "Mafra (Almada) | Circular"]],
        253: [["1201", "Alcolombal de Cima - Terrugem (Escola), via SacÃ¡rio"]],
        254: [["1255", "Rebanque - Portela de Sintra (EstaÃ§Ã£o)"]],
        255: [["1240", "Negrais - Portela de Sintra (EstaÃ§Ã£o)"]],
        281: [["2126", "Ericeira | Circular"]],
        3: [["2842", "Lisboa (C. Grande) - Vila Franca de Xira"]],
        39: [["2307", "Arcena - Vila Franca de Xira (Terminal)"]],
        40: [["2302", "Alverca - Vila Franca de Xira (Terminal)"], ["2324", "PÃ³voa de Santa Iria - Vila Franca de Xira (Terminal)"], ["2328", "PÃ³voa de Santa Iria (Qta Piedade) - Vila Franca de Xira (Terminal)"]],
        41: [["2324", "PÃ³voa de Santa Iria - Vila Franca de Xira (Terminal)"]],
        44: [["2306", "Alverca (EstaÃ§Ã£o) - Vila Franca de Xira (Terminal)"]],
        45: [["2329", "Qta da Cruz de Pau - Sobralinho"]],
        48: [["2302", "Alverca - Vila Franca de Xira (Terminal)"], ["2842", "Lisboa (C. Grande) - Vila Franca de Xira"]],
        49: [["2331", "Torre de Cima e Capelas - Vila Franca de Xira (Hospital), via Povos"], ["2332", "Vila Franca de Xira (EstaÃ§Ã£o) - Vila Franca de Xira (Hospital)"], ["2333", "Vila Franca de Xira (EstaÃ§Ã£o) - Vila Franca de Xira (Hospital), via N1"]],
        50: [["2316", "Cotovios - Vila Franca de Xira (Terminal)"]],
        53: [["2301", "Alhandra - BogalhÃ£o"], ["2308", "BogalhÃ£o - Vila Franca de Xira (Terminal)"], ["2309", "BogalhÃ£o - Vila Franca de Xira (Terminal), via Subserra"], ["2330", "Subserra - Vila Franca de Xira (Terminal)"]],
        57: [["2304", "Alverca (EstaÃ§Ã£o) - Bulhaco"]],
        58: [["2920", "A-do-MourÃ£o - Alverca (EstaÃ§Ã£o)"]],
        59: [["2921", "Alhandra - Arruda dos Vinhos"], ["2922", "Arruda dos Vinhos - Bulhaco"]],
        701: [["2806", "Lisboa (C. Grande) - Livramento"]],
        702: [["2752", "Lisboa (C. Grande) - Malveira"]],
        72: [["2841", "Lisboa (C. Grande) - Sobralinho"]],
        721: [["2122", "Enxara do Bispo - Livramento"], ["2913", "Mafra (Parque Desportivo) - Pero Negro (EstaÃ§Ã£o)"]],
        91: [["2332", "Vila Franca de Xira (EstaÃ§Ã£o) - Vila Franca de Xira (Hospital)"]]
    },
    "CM Almada": {Flexibus: [["3005", "Flexibus Almada | Circular"]]},
    "CM Oeiras": {
        "COMBUS Barcarena": [["1111", "Barcarena | Circular"]],
        "COMBUS Carnaxide": [["1109", "Carnaxide-Outurela"]],
        "COMBUS Cruz Quebrada": [["1102", "AlgÃ©s - Cruz Quebrada"]],
        "COMBUS Oeiras": [["1121", "Oeiras | Circular"]],
        "COMBUS PaÃ§o de Arcos": [["1122", "PaÃ§o de Arcos - Caxias"]],
        "COMBUS Porto Salvo": [["1125", "Porto Salvo | Circular"]],
        "COMBUS Queijas": [["1110", "Carnaxide-Queijas"]]
    },
    "Henrique Leonardo Mota": {
        201: [["2754", "Lisboa (C. Grande) - PÃ³voa da Galega"]],
        202: [["2765", "CabeÃ§o de Montachique - Lisboa (C. Grande)"]],
        204: [["2757", "Lisboa (C. Grande) - PÃ³voa da Galega, via Milharado e EN8"]],
        206: [["2746", "Lisboa (C. Grande) - Venda do Pinheiro, via Milharado e A8"]],
        209: [["2753", "Lisboa (C. Grande) - Milharado"]],
        211: [["2009", "CabeÃ§o de Montachique - Loures (Centro SaÃºde)"], ["2011", "CabeÃ§o de Montachique - Loures (Centro SaÃºde), via Murteira"]],
        219: [["2755", "Lisboa (C. Grande) - PÃ³voa da Galega, via Casais do Forno"]],
        220: [["2010", "CabeÃ§o de Montachique - Loures (Centro SaÃºde), via Bairro Novo Palhais"], ["2765", "CabeÃ§o de Montachique - Lisboa (C. Grande)"]],
        221: [["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        223: [["2744", "Lisboa (C. Grande) - PÃ³voa da Galega, via Milharado e A8"]],
        233: [["2745", "Lisboa (C. Grande) - PÃ³voa da Galega, via Murteira"]],
        234: [["2743", "Lisboa (C. Grande) - PÃ³voa da Galega, via A8 - Loures"]],
        237: [["2754", "Lisboa (C. Grande) - PÃ³voa da Galega"], ["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        239: [["2756", "Lisboa (C. Grande) - PÃ³voa da Galega, via Guerreiros e Lumiar"]],
        810: [["2776", "Guerreiros - Lisboa (C. Grande)"]],
        811: [["2017", "Guerreiros - Loures (Campo de Jogos)"], ["2776", "Guerreiros - Lisboa (C. Grande)"]],
        812: [["2777", "Guerreiros - Lisboa (C. Grande), via Lumiar"]],
        813: [["2785", "Covas de Ferro - Lisboa (C. Grande)"]],
        814: [["2003", "Ã€-dos-Moninhos - Guerreiros"], ["2760", "Ã€-dos-Moninhos - Lisboa (C. Grande)"], ["2761", "Ã€-dos-Moninhos - Lisboa (C. Grande), via Bolores"]],
        815: [["2760", "Ã€-dos-Moninhos - Lisboa (C. Grande)"]],
        816: [["2621", "Covas de Ferro - Pinheiro de Loures"]],
        818: [["2002", "A-dos-CÃ£os - Loures (Centro SaÃºde)"], ["2785", "Covas de Ferro - Lisboa (C. Grande)"]],
        819: [["2001", "A-Dos-CÃ£os - Loures (Campo de Jogos)"]],
        822: [["2620", "Covas de Ferro - Loures (Centro SaÃºde), via A-Dos-Calvos"]],
        824: [["2016", "Guerreiros - Hospital Beatriz Ã‚ngelo"]],
        826: [["2004", "Bairro de Santa Maria - Loures (Centro SaÃºde)"]],
        831: [["2620", "Covas de Ferro - Loures (Centro SaÃºde), via A-Dos-Calvos"]],
        w: [["2003", "Ã€-dos-Moninhos - Guerreiros"]]
    },
    "Isidoro Duarte": {
        207: [["2504", "Carrascal - Ponte de Lousa"]],
        210: [["2501", "Bocal - Malveira"]],
        215: [["2149", "Prezinheira - Venda do Pinheiro (Terminal)"], ["2901", "Rolia - Venda do Pinheiro (Terminal)"]],
        216: [["2108", "Cachoeira - Malveira"], ["2109", "Cachoeira - Venda do Pinheiro (Terminal)"], ["2147", "Milharado - Venda do Pinheiro (Terminal)"]],
        217: [["2148", "PÃ³voa da Galega - Venda do Pinheiro (Terminal)"], ["2151", "Venda do Pinheiro (Terminal) - Vila de Canas"]],
        241: [["2146", "Malveira (Centro SaÃºde) - Milharado (CASO) | Circular"]]
    },
    "JJ Santo AntÃ³nio": {
        410: [["2715", "Cidade Nova - Lisboa (C. Grande), via A8"]],
        411: [["2716", "Cidade Nova - Lisboa (C. Grande), via IC22"]],
        412: [["2769", "Cidade Nova - Lisboa (C. Grande)"]],
        413: [["2770", "Cidade Nova - Lisboa (C. Grande), via Lumiar"]],
        414: [["2771", "Cidade Nova - Lisboa (C. Grande), via Urb. Flores"]],
        415: [["2771", "Cidade Nova - Lisboa (C. Grande), via Urb. Flores"]],
        416: [["2012", "Conventinho - Sto. Ant. Cavaleiros"], ["2020", "Sto. Ant. Cavaleiros | Circular, via Hospital Beatriz Ã‚ngelo"]],
        417: [["2014", "Escola Maria Veleda - Frielas"]],
        421: [["2717", "Lisboa (C. Grande) - Torres da Bela Vista, via IC22"]],
        422: [["2772", "Lisboa (C. Grande) - Torres da Bela Vista"]],
        423: [["2773", "Lisboa (C. Grande) - Torres da Bela Vista, via Lumiar"]],
        424: [["2772", "Lisboa (C. Grande) - Torres da Bela Vista"]],
        425: [["2773", "Lisboa (C. Grande) - Torres da Bela Vista, via Lumiar"]],
        426: [["2019", "Hospital Beatriz Ã‚ngelo - Torres da Bela Vista, via Escola JosÃ© Cardoso Pires"], ["2772", "Lisboa (C. Grande) - Torres da Bela Vista"]],
        427: [["2018", "Hospital Beatriz Ã‚ngelo - Torres da Bela Vista"], ["2773", "Lisboa (C. Grande) - Torres da Bela Vista, via Lumiar"]],
        430: [["2015", "Flamenga - Torres da Bela Vista"]],
        431: [["2013", "Conventinho - Torres da Bela Vista"]],
        440: [["2027", "Loures | Circular, via Cidade Nova"]],
        441: [["2028", "Loures | Circular, via Ponte Frielas"]],
        450: [["2774", "Frielas - Lisboa (C. Grande)"]],
        451: [["2775", "Frielas - Lisboa (C. Grande), via Zona Industrial"]],
        460: [["2720", "Lisboa (C. Grande) - Loures (Bairro Urmeira)"]],
        470: [["2650", "Cidade Nova - Costa da Caparica"]]
    },
    Nova: {
        Nova: [["1001", "Alfragide (Estrada do Seminario) - Reboleira (EstaÃ§Ã£o)"], ["1002", "Alfragide (Igreja da Divina MisericÃ³rdia) - Amadora"], ["1008", "Amadora Este | Circular"], ["1009", "Amadora Hospital | Circular"], ["1010", "Brandoa (Esc. SecundÃ¡ria) - Casal da Mira"], ["1012", "Brandoa | Circular"], ["1013", "Casas do Lago - Damaia (Escola Doutor Azevedo Neves)"], ["1014", "Casas do Lago - Vila ChÃ£"], ["1015", "Reboleira | Circular"], ["1228", "MassamÃ¡ (EstaÃ§Ã£o) | Circular, via Belas"], ["1232", "Rio de Mouro (EstaÃ§Ã£o) | Circular, via Mem Martins"], ["1242", "Cabrela - VÃ¡rzea de Colares"], ["1244", "Casal de Cambra - Portela de Sintra (EstaÃ§Ã£o)"], ["1501", "Alfragide - Reboleira (EstaÃ§Ã£o) | Circular"], ["1506", "Amadora Hospital | Circular, via Alfragide"], ["1509", "Amadora (Hospital) - Casal de Cambra (C. SaÃºde)"], ["1516", "Casal de Cambra - Monte AbraÃ£o (EstaÃ§Ã£o)"], ["1520", "AlgÃ©s (Terminal) - CacÃ©m (EstaÃ§Ã£o)"], ["1521", "AlgÃ©s (Terminal) - CacÃ©m (EstaÃ§Ã£o), via A5"], ["1605", "Carnaxide (Av. JoÃ£o Paulo II) - Nova SBE"], ["1606", "Carnaxide (Av. JoÃ£o Paulo II) - Nova SBE, via Terrugem"], ["1612", "CacÃ©m (EstaÃ§Ã£o) - Carcavelos (EstaÃ§Ã£o)"], ["1719", "Casal de Cambra - Lisboa (C. Militar)"], ["1731", "CacÃ©m (EstaÃ§Ã£o) - Hospital SÃ£o Francisco Xavier"], ["2032", "SacavÃ©m (EstaÃ§Ã£o) - Santa Iria da AzÃ³ia (EstaÃ§Ã£o)"], ["2033", "Bobadela - SÃ£o JoÃ£o da Talha | Circular"], ["2034", "Santa Iria da AzÃ³ia | Circular"], ["2120", "Enxara do Bispo - Ericeira (Terminal)"], ["2121", "Enxara do Bispo - Gradil"], ["2136", "Jerumelo - Mafra"], ["2141", "Mafra - Ribamar"], ["2204", "CaneÃ§as - FamÃµes"], ["2210", "Jardim da Amoreira - Odivelas"], ["2305", "Alverca (EstaÃ§Ã£o) - Malvarosa | Circular"], ["2315", "Castanheira do Ribatejo | Circular"], ["2322", "Bolonha - PÃ³voa de Santa Iria (EstaÃ§Ã£o)"], ["2505", "Loures (C. Comercial) - Malveira"], ["2506", "Milharado (CASO) - Ponte de Lousa"], ["2511", "Bairro dos CTT - Loures (C. Comercial)"], ["2512", "Bucelas - Sr. Roubado (Metro), via Ramada"], ["2534", "Loures - Santa Iria da AzÃ³ia"], ["2535", "Loures - Vila Franca de Xira"], ["2539", "SacavÃ©m (EstaÃ§Ã£o) - Santa Iria da AzÃ³ia (EstaÃ§Ã£o), via Portela da AzÃ³ia"], ["2540", "Santa Iria da AzÃ³ia - Vialonga"], ["2610", "Odivelas (Metro) - UBBO"], ["2611", "UBBO - Ramada"], ["2625", "Mafra - SÃ£o JoÃ£o das Lampas"], ["2709", "Camarate | Circular"], ["2719", "Lisboa (C. Grande) - Loures"], ["2724", "Lisboa (Oriente) - Loures"], ["2726", "Lisboa (Oriente) - Loures, via SacavÃ©m"], ["2733", "Loures - Moscavide (Metro)"], ["2734", "Prior Velho - SacavÃ©m (EstaÃ§Ã£o)"], ["3002", "Almada (Parque Urbano) - Pragal (EstaÃ§Ã£o)"], ["3006", "Aroeira | Circular"], ["3009", "Cacilhas (terminal - Trafaria (Terminal)"], ["3016", "Charneca da Caparica - Lazarim"], ["3017", "Charneca da Caparica - Pragal (EstaÃ§Ã£o)"], ["3019", "Charneca da Caparica - Trafaria (Terminal)"], ["3020", "Charneca da Caparica | Circular"], ["3021", "Costa da Caparica - Monte da Caparica (FCT)"], ["3025", "Costa da Caparica (Terminal) - Pragal (EstaÃ§Ã£o), via IC20"], ["3028", "Lazarim | Circular"], ["3029", "Marco CabaÃ§o | Circular"], ["3031", "Monte da Caparica - Quintinha"], ["3033", "Monte da Caparica | Circular"], ["3035", "Pragal (EstaÃ§Ã£o) - Quinta do Texugo"], ["3036", "Pragal (EstaÃ§Ã£o) - Vale Flores"], ["3037", "Quintinha | Circular"], ["3106", "Coina (EstaÃ§Ã£o) - FernÃ£o Ferro"], ["3115", "Marisol - Foros de Amora (EstaÃ§Ã£o), via Corroios (EstaÃ§Ã£o)"], ["3116", "Marisol - Seixal (Terminal Fluvial), via Corroios (EstaÃ§Ã£o)"], ["3117", "Marisol (Valadares) - Foros de Amora (EstaÃ§Ã£o)"], ["3118", "Marisol (Valadares) - Seixal (Terminal Fluvial)"], ["3121", "Seixal | Circular"], ["3219", "Sesimbra (R. Palames) - Sesimbra (Terminal)"], ["3222", "Quinta do Conde | Circular"], ["3503", "Almada Forum - Vale de MilhaÃ§os"], ["3505", "Cacilhas (Terminal) - Corroios (EstaÃ§Ã£o)"], ["3540", "Alfarim - Coina (EstaÃ§Ã£o)"], ["3541", "Coina (EstaÃ§Ã£o) - FernÃ£o Ferro, via Casal do Sapo e Pinhal do General"], ["3542", "Coina (EstaÃ§Ã£o) - Praia do Meco"], ["3544", "Coina (EstaÃ§Ã£o) - Sesimbra (Terminal)"], ["3601", "Barreiro - Cova da Piedade (Metro)"], ["3615", "Barreiro - Seixal"], ["3625", "Barreiro - Sesimbra (Terminal)"], ["3630", "AzeitÃ£o - Penalva (EstaÃ§Ã£o)"], ["3701", "Almada (Centro Sul) - AlgÃ©s (Terminal)"], ["3706", "Charneca da Caparica - Lisboa (Sete Rios), via FeijÃ³"], ["3708", "Costa da Caparica (Terminal) - Lisboa (C. SodrÃ©)"], ["4001", "Alcochete | Circular"], ["4002", "SÃ£o Francisco | Circular"], ["4208", "Montijo (Terminal RodoviÃ¡rio) - Sarilhos Grandes (Estr. 4 Marcos)"], ["4211", "Craveiras - PegÃµes | Circular"], ["4303", "Palmela | Circular"], ["4304", "Palmela (Terminal) - Penalva"], ["4308", "Palmela (Terminal) - Pinhal Novo (EstaÃ§Ã£o)"], ["4313", "Cabanas - Penalva"], ["4401", "Cachofarra - SetÃºbal (Hospital)"], ["4404", "Interfaces SetÃºbal | Circular"], ["4405", "Livramento-Montebelo | Circular"], ["4409", "Manteigadas - Viso"], ["4410", "Manteigadas (Esc. Profissional) - SetÃºbal (Alegro)"], ["4427", "SetÃºbal (Bela Vista) - SetÃºbal (Mercado)"], ["4429", "SetÃºbal (Centro SaÃºde) - SetÃºbal (Mercado)"], ["4430", "SetÃºbal (Hospital) - SetÃºbal (MontalvÃ£o)"], ["4432", "SetÃºbal (ITS) - Vale de Choupo"], ["4435", "Biscainho - FaralhÃ£o"], ["4443", "SetÃºbal (PolitÃ©cnico) - Praias do Sado"], ["4460", "AzeitÃ£o | Circular"], ["4475", "Portinho da ArrÃ¡bida - Viso"], ["4503", "Atalaia - Jardia"], ["4612", "Bairro dos Marinheiros - Palmela (Terminal)"], ["4621", "Moita - Seixal (Terminal Fluvial)"]]
    },
    "RodoviÃ¡ria de Lisboa": {
        "1/3": [["2217", "Odivelas (Metro) | Circular"]],
        153: [["2303", "Alverca (EstaÃ§Ã£o) - Arcena"]],
        201: [["2207", "CaneÃ§as (Esc. SecundÃ¡ria) - Sr. Roubado (Metro), via Odivelas (Centro)"], ["2812", "CaneÃ§as (Esc. SecundÃ¡ria) - Lisboa (C. Grande), via Sr. Roubado (Metro)"]],
        202: [["2522", "Montemor - Sr. Roubado (Metro)"]],
        203: [["2830", "Casal Bispo - Lisboa (C. Militar)"], ["2831", "Casal Bispo - Pontinha (Metro)"]],
        205: [["2819", "Lisboa (C. Militar) - Sr. Roubado (Metro)"], ["2824", "Pontinha (Metro) - Sr. Roubado (Metro)"]],
        206: [["2519", "Loures (C.C. Continente) - Odivelas (Colinas do Cruzeiro)"], ["2521", "Loures (Campo de Jogos) - Odivelas (Metro)"], ["2601", "Loures (C.C. Continente) - Reboleira (Metro)"], ["2780", "Loures (C.C. Continente) - Pontinha (Metro)"], ["2820", "Odivelas (Colinas do Cruzeiro) - Pontinha (Metro)"], ["2821", "Odivelas (Metro) - Pontinha (Metro)"]],
        207: [["2219", "Odivelas (Metro) | Circular, via PÃ³voa de Santo AdriÃ£o"]],
        208: [["2203", "Arroja | Circular, via Odivelas (Metro)"]],
        209: [["2203", "Arroja | Circular, via Odivelas (Metro)"]],
        210: [["1709", "CaneÃ§as - Lisboa (C. Militar)"], ["1710", "CaneÃ§as - Pontinha (Metro)"]],
        211: [["2215", "Odivelas (Metro) - Ramada (Bairro Bons Dias)"], ["2816", "Lisboa (C. Grande) - Ramada (Bairro Bons Dias)"]],
        212: [["2208", "CaneÃ§as (Jardim) - Vale Nogueira"]],
        213: [["2206", "CaneÃ§as (Esc. SecundÃ¡ria) - Sr. Roubado (Metro)"]],
        214: [["2515", "Casal Paradela - Odivelas (Metro)"], ["2768", "Casal Paradela - Lisboa (C. Grande)"]],
        215: [["2513", "CaneÃ§as (Esc. SecundÃ¡ria) - Loures (Lg Marcos RomÃ£o Reis JÃºnior)"], ["2605", "CacÃ©m (EstaÃ§Ã£o) - Loures (Lg Marcos RomÃ£o Reis JÃºnior)"], ["2606", "CacÃ©m (EstaÃ§Ã£o) - CaneÃ§as (Casa da Cultura)"]],
        216: [["2223", "Sr. Roubado (Metro) | Circular, via Casal Novo"]],
        220: [["1636", "Bairro Arco Maria Teresa - Dona Maria"], ["1637", "Casal de Cambra - Vale de Lobos"]],
        221: [["1635", "Almargem do Bispo (Centro de SaÃºde) - CaneÃ§as"]],
        222: [["2811", "CaneÃ§as (Bairro do Monte Verde) - Lisboa (C. Militar)"], ["2823", "Pedernais (Bairro Girassol) - Pontinha (Metro)"]],
        223: [["2813", "Casal Novo - Lisboa (C. Militar)"], ["2814", "Casal Novo - Pontinha (Metro)"]],
        224: [["1711", "CaneÃ§as (Esc. SecundÃ¡ria) - Pontinha (Metro)"]],
        225: [["2216", "Odivelas (Metro) - Ramada (R. Heliodoro Salgado)"], ["2524", "Odivelas (Metro) | Circular, via Hospital Beatriz Ã‚ngelo"]],
        226: [["2202", "Arroja - Sr. Roubado (Metro)"], ["2810", "Arroja - Lisboa (C. Grande)"]],
        227: [["2701", "Pontinha (Metro) - Vale Grande"]],
        228: [["2781", "Loures (C.C. Continente) - Pontinha (Metro), via Ramada"], ["2815", "Jardim da Amoreira - Pontinha (Metro)"], ["2818", "Lisboa (C. Militar) - Serra da Amoreira"]],
        229: [["2218", "Odivelas (Metro) | Circular, via Colinas do Cruzeiro"]],
        230: [["2631", "Casal de Cambra (C. SaÃºde) - Odivelas (Metro)"], ["2632", "Casal de Cambra (C. SaÃºde) - Odivelas (Metro), via Bairro Sol Nascente"]],
        231: [["1709", "CaneÃ§as - Lisboa (C. Militar)"]],
        233: [["2850", "Costa da Caparica - PÃ³voa de Santo AdriÃ£o (Parque Urbano)"]],
        235: [["2214", "Odivelas (Metro) - PÃ³voa de Santo AdriÃ£o (Parque Urbano)"], ["2221", "PÃ³voa de Santo AdriÃ£o (Parque Urbano) - Sr. Roubado (Metro)"], ["2516", "Casal Paradela - Sr. Roubado (Metro)"]],
        236: [["2832", "Casal Novo - Lisboa (C. Militar), via Casal de Cambra"], ["2833", "Pontinha (Metro) | Circular, via Casal Novo"]],
        237: [["2212", "Odivelas (C. Comercial) | Circular"]],
        238: [["2518", "IKEA Loures - Sr. Roubado (Metro)"]],
        240: [["2220", "Olival Basto | Circular, via PÃ³voa de Santo AdriÃ£o e Odivelas (Metro)"]],
        241: [["2222", "Ramada | Circular"]],
        300: [["2714", "Lisboa (C. Grande) - SacavÃ©m (Jardim)"]],
        301: [["2023", "Loures (C.C. Continente) - SacavÃ©m (ClÃ­nica Sto AntÃ³nio)"], ["2024", "Loures (EDP) - SacavÃ©m (ClÃ­nica Sto AntÃ³nio)"], ["2725", "Lisboa (Oriente) - Loures (C.C. Continente)"]],
        302: [["2732", "Lisboa (PÃ§ JosÃ© QueirÃ³s) | Circular, via SacavÃ©m e Camarate"]],
        303: [["2029", "Moscavide (Metro) | Circular, via Portela"]],
        305: [["2005", "Bairro do Espinhal - SacavÃ©m (EstaÃ§Ã£o)"], ["2025", "Loures (Lg Marcos RomÃ£o Reis JÃºnior) - SacavÃ©m (EstaÃ§Ã£o)"], ["2704", "Bairro Espinhal - Lisboa (Oriente)"], ["2727", "Lisboa (Oriente) - Loures, via Unhos"]],
        306: [["2036", "Urbana de Camarate | Circular"]],
        307: [["2735", "Urbana de SacavÃ©m | Circular, via Prior Velho"]],
        308: [["2731", "Lisboa (Oriente) - SacavÃ©m | Circular, via Portela"]],
        309: [["2708", "CabeÃ§o Aguieira - Lisboa (Oriente)"]],
        310: [["2711", "Charneca - Lisboa (Oriente)"], ["2712", "Charneca do Lumiar - SacavÃ©m (Jardim), via Bairro de Santiago"]],
        311: [["2710", "Catujal (Bairro Alto MoÃ­nho) - Lisboa (C. Grande)"]],
        312: [["2713", "Lisboa (C. Grande) | Circular, via SacavÃ©m e ApelaÃ§Ã£o"]],
        313: [["2713", "Lisboa (C. Grande) | Circular, via SacavÃ©m e ApelaÃ§Ã£o"]],
        315: [["2703", "Bairro de SÃ£o JosÃ© - Lisboa (C. Grande) | Circular"]],
        316: [["2730", "Lisboa (Oriente) - Santa Iria da AzÃ³ia"]],
        317: [["2728", "Bairro Covina - Lisboa (Oriente)"]],
        318: [["2729", "Lisboa (Oriente) - Portela da AzÃ³ia"]],
        319: [["2791", "Alverca (Z. Industrial) - Lisboa (C. Grande)"]],
        320: [["2790", "Alverca (EstaÃ§Ã£o) - Lisboa (C. Grande)"], ["2793", "Forte da Casa - Lisboa (C. Grande)"]],
        321: [["2722", "Lisboa (C. Grande) - Via Rara"]],
        322: [["2319", "Forte da Casa | Circular"]],
        323: [["2318", "Forte da Casa - PÃ³voa de Santa Iria (EstaÃ§Ã£o) | Circular"]],
        324: [["2323", "PÃ³voa de Santa Iria | Circular"]],
        325: [["2536", "PÃ³voa de Santa Iria - SalvaÃ§Ã£o | Circular"]],
        326: [["2323", "PÃ³voa de Santa Iria | Circular"]],
        "327 (adaptada)": [["2026", "Loures | Circular"]],
        328: [["2323", "PÃ³voa de Santa Iria | Circular"]],
        329: [["2721", "Lisboa (C. Grande) - Periscoxe"], ["2795", "Lisboa (C. Grande) - Quinta da Piedade"]],
        330: [["2794", "Forte da Casa - Lisboa (Oriente)"]],
        331: [["2764", "Bucelas - Lisboa (C. Grande), via S. JulÃ£o do Tojal"], ["2767", "Casainhos - Lisboa (C. Grande), via Lumiar"]],
        332: [["2037", "Zambujal | Circular"]],
        333: [["2723", "Lisboa (C. Grande) - Zambujal"]],
        334: [["2718", "Infantado - Lisboa (C. Grande)"]],
        335: [["2503", "Bucelas - Santo AntÃ£o do Tojal"], ["2510", "Bucelas - Sr. Roubado (Metro)"], ["2514", "Casainhos - Sr. Roubado (Metro), via Loures (C.Comercial)"], ["2750", "Bucelas - Lisboa (C. Grande), via CabeÃ§o de Montachique e Odivelas (Metro)"], ["2766", "Casainhos - Lisboa (C. Grande)"], ["2767", "Casainhos - Lisboa (C. Grande), via Lumiar"], ["2779", "Lisboa (C. Grande) - Santo AntÃ£o do Tojal, via Infantado e Loures (C.Comercial)"]],
        336: [["2762", "Bucelas - Lisboa (C. Grande), via A8"], ["2763", "Bucelas - Lisboa (C. Grande), via Lumiar"], ["2767", "Casainhos - Lisboa (C. Grande), via Lumiar"]],
        337: [["2021", "Infantado - Santo AntÃ£o do Tojal"], ["2022", "Loures - Monte EsperanÃ§a"], ["2525", "Santo AntÃ£o do Tojal - Sr. Roubado (Metro)"], ["2778", "Lisboa (C. Grande) - Santo AntÃ£o do Tojal"]],
        338: [["2021", "Infantado - Santo AntÃ£o do Tojal"]],
        340: [["2502", "Bucelas - Malveira"]],
        342: [["2007", "Bucelas - Mato da Cruz"], ["2530", "Alverca (EstaÃ§Ã£o) - Bucelas, via Malvarosa"]],
        344: [["2706", "Bucelas - Lisboa (C. Grande)"], ["2707", "Bucelas - Lisboa (C. Grande), via SÃ£o JuliÃ£o do Tojal e A8"]],
        345: [["2792", "Arcena - Lisboa (Oriente)"]],
        346: [["2531", "Alverca (EstaÃ§Ã£o) - Granja, via Casal das Areias"], ["2533", "Alverca (EstaÃ§Ã£o) - Santo AntÃ£o do Tojal, via Casal das Areias"]],
        347: [["2840", "Arcena - Lisboa (Oriente), via A1"]],
        348: [["2537", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Santo AntÃ£o do Tojal"]],
        349: [["2317", "Fonte Santa - PÃ³voa de Santa Iria (EstaÃ§Ã£o), via Morgado"], ["2326", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Vialonga"]],
        350: [["2325", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Verdelho Ruivo"]],
        351: [["2327", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Vialonga | Circular"]],
        353: [["2797", "Lisboa (C. Grande) - Vialonga, via A8"], ["2798", "Lisboa (C. Grande) - Vialonga, via A9 e A8"]],
        354: [["2796", "Lisboa (C. Grande) - Vialonga"]],
        355: [["2651", "Bucelas - Costa da Caparica (Terminal)"]],
        357: [["2008", "Bucelas - RomÃ£o Charneca | Circular"]],
        358: [["2006", "Bemposta - Bucelas | Circular"]],
        360: [["2532", "Alverca (EstaÃ§Ã£o) - Loures"]],
        363: [["2035", "Santo AntÃ£o do Tojal | Circular"]],
        365: [["2520", "Loures (C.Comercial) - Sr. Roubado (Metro)"]],
        370: [["2537", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Santo AntÃ£o do Tojal"], ["2538", "PÃ³voa de Santa Iria (EstaÃ§Ã£o) - Santo AntÃ£o do Tojal, via Zambujal"]],
        4: [["2209", "Casal Bispo - Sr. Roubado (Metro)"]],
        5: [["2630", "CaneÃ§as (Esc. SecundÃ¡ria) | Circular"]],
        701: [["2652", "Costa da Caparica (Terminal) - Forte da Casa"]],
        710: [["2031", "SacavÃ©m (C. SaÃºde) | Circular"]],
        711: [["2030", "SacavÃ©m (C. SaÃºde) - Fetais | Circular"]],
        750: [["2705", "Bairro Espinhal - Lisboa (Oriente) | Circular"]],
        901: [["2207", "CaneÃ§as (Esc. SecundÃ¡ria) - Sr. Roubado (Metro), via Odivelas (Centro)"], ["2812", "CaneÃ§as (Esc. SecundÃ¡ria) - Lisboa (C. Grande), via Sr. Roubado (Metro)"]],
        905: [["2817", "Lisboa (C. Militar) - Odivelas (Metro)"], ["2822", "Odivelas (Metro) - Pontinha (Metro), via Serra da Luz"]],
        913: [["2205", "CaneÃ§as (Esc. SecundÃ¡ria) - Odivelas (Metro)"]],
        916: [["2213", "Odivelas (Metro) - FamÃµes | Circular"]],
        925: [["2211", "Jardim da Amoreira - Odivelas (Metro)"], ["2517", "Hospital Beatriz Ã‚ngelo - Odivelas (Metro) | Circular"]],
        926: [["2201", "Arroja - Odivelas (Metro)"]],
        931: [["2702", "Lisboa (C. Grande) - Pontinha (Metro)"]],
        934: [["2215", "Odivelas (Metro) - Ramada (Bairro Bons Dias)"], ["2523", "Odivelas (Metro) - Montemor"]]
    },
    "RodoviÃ¡ria do Alentejo": {
        8080: [["4905", "Faias - Vendas Novas"]],
        8902: [["4902", "Landeira - PegÃµes"], ["4906", "SetÃºbal (ITS) - Vendas Novas, via Landeira"]]
    },
    Scotturb: {
        403: [["1624", "Cascais (Terminal) - Portela de Sintra (EstaÃ§Ã£o), via AzÃ³ia e AlmoÃ§ageme"]],
        "403 (Adaptado)": [["1253", "Circuito Cabo da Roca - Portela de Sintra"]],
        417: [["1623", "Cascais (Terminal) - Portela de Sintra (EstaÃ§Ã£o)"], ["1626", "CascaiShopping - Portela de Sintra (EstaÃ§Ã£o)"]],
        418: [["1621", "Bairro da Cruz Vermelha - Portela de Sintra (EstaÃ§Ã£o)"], ["1626", "CascaiShopping - Portela de Sintra (EstaÃ§Ã£o)"], ["1629", "Estoril (EstaÃ§Ã£o) - Portela de Sintra (EstaÃ§Ã£o)"], ["1630", "Estoril (EstaÃ§Ã£o) - Portela de Sintra (EstaÃ§Ã£o), via Monte Estoril e Amoreira"]],
        433: [["1252", "Portela de Sintra (EstaÃ§Ã£o) | Circular"]],
        437: [["1206", "AlgueirÃ£o-Mem Martins (EstaÃ§Ã£o) - Portela de Sintra (EstaÃ§Ã£o), via Tribunal"]],
        "439 (Adaptado)": [["1250", "Portela de Sintra (EstaÃ§Ã£o) - Praia Grande"]],
        "440 (Adaptado)": [["1247", "Fontanelas - Portela de Sintra (EstaÃ§Ã£o), via Janas"]],
        441: [["1241", "AlmoÃ§ageme (Av. Dr. B. Vasc. X R. Sto AndrÃ©) - Portela de Sintra (EstaÃ§Ã£o)"]],
        "441 (Adaptado)": [["1254", "Circuito Praias - Portela de Sintra"]],
        442: [["1246", "Fontanelas - Portela de Sintra (EstaÃ§Ã£o), via Casal da Granja"], ["1248", "Portela de Sintra (EstaÃ§Ã£o) - Praia das MaÃ§Ã£s (Piscinas)"]],
        443: [["1245", "Catribana - Portela de Sintra (EstaÃ§Ã£o)"]],
        444: [["1249", "Portela de Sintra (EstaÃ§Ã£o) - Praia do Magoito"]],
        445: [["1243", "Casais da Cabrela - Portela de Sintra (EstaÃ§Ã£o)"], ["1632", "Carvalhal - Portela de Sintra (EstaÃ§Ã£o)"]],
        446: [["1234", "ShopBus Mira Sintra"]],
        447: [["1208", "Almargem do Bispo (CemitÃ©rio) - Portela de Sintra (EstaÃ§Ã£o)"], ["1235", "Covas de Ferro - MercÃªs (EstaÃ§Ã£o)"]],
        "447 (adaptada)": [["1231", "Mem Martins (Esc. Ferreira Castro) - Coutinho Afonso"]],
        448: [["1214", "CacÃ©m (EstaÃ§Ã£o) - Alegro Sintra"], ["1223", "ShopBus CacÃ©m"]],
        450: [["1210", "Bairro Fitares - Mem Martins (Esc. SecundÃ¡ria)"]],
        455: [["1625", "Cascais (Terminal) - Rio de Mouro (EstaÃ§Ã£o)"]],
        456: [["1627", "CascaiShopping - Rio de Mouro (EstaÃ§Ã£o)"], ["1628", "Estoril (EstaÃ§Ã£o) -  Rio de Mouro (EstaÃ§Ã£o)"], ["1631", "Estoril (EstaÃ§Ã£o) - Rio de Mouro (EstaÃ§Ã£o) | Direta"]],
        458: [["1205", "AlgueirÃ£o-Mem Martins (EstaÃ§Ã£o) - Mem Martins (Esc. SecundÃ¡ria)"]],
        460: [["1202", "Alegro Sintra - Mem Martins | Circular"], ["1203", "Alegro Sintra - Rio de Mouro (EstaÃ§Ã£o)"], ["1207", "AlgueirÃ£o-Mem Martins (EstaÃ§Ã£o) - Rio de Mouro (EstaÃ§Ã£o)"], ["1251", "Portela de Sintra (EstaÃ§Ã£o) - Rio de Mouro (EstaÃ§Ã£o)"]],
        463: [["1613", "CacÃ©m (EstaÃ§Ã£o) - Oeiras (EstaÃ§Ã£o), via Trajouce"], ["1618", "AbÃ³boda (Auto BarÃ£o) - CacÃ©m (EstaÃ§Ã£o)"], ["1622", "CacÃ©m (EstaÃ§Ã£o) - Carcavelos (EstaÃ§Ã£o), via Trajouce"]],
        467: [["1614", "Carcavelos (EstaÃ§Ã£o) - Portela de Sintra (EstaÃ§Ã£o)"], ["1619", "AbÃ³boda (Auto BarÃ£o) - Portela de Sintra (EstaÃ§Ã£o)"]],
        468: [["1615", "Carcavelos (EstaÃ§Ã£o) - Rio de Mouro (EstaÃ§Ã£o)"], ["1620", "AbÃ³boda (Auto BarÃ£o) - Rio de Mouro (EstaÃ§Ã£o)"]],
        470: [["1607", "ConceiÃ§Ã£o da AbÃ³boda - Oeiras (EstaÃ§Ã£o)"], ["1608", "Oeiras (EstaÃ§Ã£o) - Taguspark"], ["1609", "Oeiras (EstaÃ§Ã£o) - TalaÃ­de (Igreja)"]],
        471: [["1604", "Carcavelos (EstaÃ§Ã£o) - Parede (Terminal)"]],
        479: [["1604", "Carcavelos (EstaÃ§Ã£o) - Parede (Terminal)"]]
    },
    SulFertagus: {
        "1A": [["3101", "Amora - Foros de Amora (EstaÃ§Ã£o)"]],
        "1C": [["3516", "Charneca da Caparica - Corroios (EstaÃ§Ã£o)"]],
        "1F": [["3112", "Fogueteiro (EstaÃ§Ã£o) - Seixal (Terminal Fluvial), via Paio Pires"]],
        "1N": [["3620", "Coina (EstaÃ§Ã£o) - Quinta do Conde"]],
        "1P": [["3027", "Hospital Garcia de Orta - Sobreda"]],
        "2A": [["3108", "Fogueteiro - Foros de Amora (EstaÃ§Ã£o)"]],
        "2C (Adaptado)": [["3506", "Cacilhas (Terminal) - Corroios (EstaÃ§Ã£o), via Miratejo"]],
        "2F": [["3113", "Fogueteiro (EstaÃ§Ã£o) - Seixal (Terminal Fluvial), via Quinta do Cabral"]],
        "2N": [["3626", "Coina (EstaÃ§Ã£o) - Vila Fresca de AzeitÃ£o"]],
        "2ND": [["3543", "Coina (EstaÃ§Ã£o) - Quinta do Conde, via Estrada de Coina"]],
        "3C": [["3517", "Chegadinho - Corroios (EstaÃ§Ã£o)"]],
        "3F": [["3105", "FernÃ£o Ferro - Fogueteiro (EstaÃ§Ã£o)"]],
        "4F": [["3109", "Fogueteiro (EstaÃ§Ã£o) - Parque Empresarial do Seixal"]]
    },
    "Transportes Sul do Tejo": {
        101: [["3001", "Almada (Cristo Rei) - Cacilhas (Terminal)"]],
        102: [["3003", "Almada Forum - Cacilhas (Terminal)"]],
        106: [["3007", "Bairro Fundo Fomento - Cacilhas (Terminal)"]],
        108: [["3103", "Corroios (EstaÃ§Ã£o) - Paio Pires (Farinheiras)"]],
        110: [["3513", "Cacilhas (Terminal) - Santa Marta do Pinhal"]],
        112: [["3114", "Foros de Amora - Paio Pires (Quinta FlamÃ¢ncia)"]],
        114: [["3508", "Cacilhas (Terminal) - Paio Pires (Centro)"]],
        116: [["3102", "Aroeira - Paio Pires (Quinta FlamÃ¢ncia)"], ["3122", "Verdizela - Cruz de Pau"], ["3521", "Cruz de Pau - Fonta da Telha"], ["3523", "Fonte da Telha - Paio Pires (Quinta FlamÃ¢ncia), via Seixal (Terminal Fluvial) e Foros de Amora (EstaÃ§Ã£o)"]],
        117: [["3014", "Cacilhas (terminal) - Raposeira"]],
        120: [["3515", "Caparica (Pilotos) -  Corroios"]],
        121: [["3526", "Laranjeiro - Pinheirinho"]],
        "123 (Adaptado)": [["3013", "Cacilhas (Terminal) - Monte da Caparica"]],
        "124 (Adaptado)": [["3022", "Costa da Caparica (Terminal) - Hospital Garcia de Orta"]],
        "126 (Adaptado)": [["3507", "Cacilhas (Terminal) - Marisol"], ["3524", "Hospital Garcia de Orta - Marisol"]],
        "127 (Adaptado)": [["3012", "Cacilhas (Terminal) - Fonte da Telha"]],
        "129 (Adaptado)": [["3030", "Fonte da Telha - Monte da Caparica (FCT)"]],
        "130 (Adaptado)": [["3030", "Fonte da Telha - Monte da Caparica (FCT)"]],
        135: [["3011", "Cacilhas (Terminal) - Costa da Caparica"]],
        137: [["3110", "Fogueteiro (EstaÃ§Ã£o) - Redondos"], ["3120", "Redondos - Seixal (Terminal Fluvial)"]],
        139: [["3519", "Costa da Caparica (Terminal) - Corroios (EstaÃ§Ã£o)"]],
        143: [["3518", "Corroios (EstaÃ§Ã£o) - Vale de Figueira"]],
        "145 (Adaptado)": [["3010", "Cacilhas (Terminal) - Charneca da Caparica"]],
        146: [["3008", "BanÃ¡tica - Quintinha"]],
        "146 (Adaptado)": [["3032", "Monte da Caparica (FCT) - Quinta do Texugo"], ["3034", "Porto BrandÃ£o (Terminal) - Quinta do Texugo"]],
        149: [["3512", "Cacilhas (Terminal) - Quinta Princesa"]],
        151: [["3704", "Charneca da Caparica - Lisboa (M. Pombal)"]],
        153: [["3710", "Costa da Caparica (Terminal) - Lisboa (Sete Rios)"]],
        155: [["3709", "Costa da Caparica (Terminal) - Lisboa (M. Pombal)"]],
        158: [["3711", "Monte da Caparica (FCT) - Lisboa (Sete Rios)"]],
        159: [["3707", "Charneca da Caparica - Lisboa (Sete Rios), via Sobreda"], ["3716", "Lisboa (Sete Rios) - Marisol"]],
        "159 (adaptada)": [["3501", "Almada Forum - Marisol, via Sobreda"]],
        160: [["3703", "Almada (Parque Urbano) - Lisboa (Sete Rios)"]],
        161: [["3710", "Costa da Caparica (Terminal) - Lisboa (Sete Rios)"]],
        162: [["3717", "Lisboa (Sete Rios) - Quinta do Brasileiro"]],
        163: [["3520", "Costa da Caparica (Terminal) - Quinta do Brasileiro"]],
        167: [["3023", "Costa da Caparica (terminal) - Laranjeiro"]],
        169: [["3715", "Lisboa (M. Pombal) - Santa Marta do Pinhal"]],
        171: [["3015", "Charneca da Caparica - Cova do Vapor"]],
        172: [["3522", "Fonte da Telha - Paio Pires (Centro)"]],
        "174 (Adaptado)": [["3024", "Costa da Caparica (Terminal) - Pragal (EstaÃ§Ã£o)"]],
        "175 (adaptada)": [["3501", "Almada Forum - Marisol, via Sobreda"]],
        176: [["3702", "Almada (Parque Urbano) - Lisboa (C. UniversitÃ¡ria)"]],
        "179(adaptada)": [["3004", "Almada Forum - Marisol"]],
        180: [["3018", "Charneca da Caparica - Sobreda"]],
        "181 (adaptada)": [["3525", "Hospital Garcia de Orta - Miratejo"]],
        182: [["3026", "Cova da Piedade - Hospital Garcia de Orta"]],
        184: [["3111", "Fogueteiro (EstaÃ§Ã£o) - Seixal (Terminal Fluvial)"]],
        "190 (adaptada)": [["3705", "Charneca da Caparica - Lisboa (Sete Rios)"]],
        191: [["3514", "Cacilhas (Terminal) - Vale de MilhaÃ§os"], ["3104", "Corroios (EstaÃ§Ã£o) - Vale de MilhaÃ§os"]],
        192: [["3511", "Cacilhas (Terminal) - Pinheirinho"]],
        195: [["3119", "Pinhal Conde Cunha - Seixal (Terminal Fluvial)"]],
        196: [["3510", "Cacilhas (Terminal) - Pilotos"]],
        197: [["3504", "Bairro Fundo Fomento - Quintinha"]],
        198: [["3502", "Almada Forum - Paio Pires (Centro)"], ["3527", "Monte da Caparica (FCT) - Paio Pires (Bairro Cucena)"], ["3528", "Monte da Caparica (FCT) - Paio Pires (Centro)"]],
        199: [["3509", "Cacilhas (Terminal) - Paio Pires (Centro), via Seixal (Terminal Fluvial) e Amora"]],
        201: [["3203", "Azoia - Sesimbra (Terminal)"], ["3204", "Azoia - Sesimbra (Terminal), via Serra da Azoia"], ["3205", "Cabo Espichel - Sesimbra (Terminal)"]],
        203: [["3536", "Cacilhas (Terminal) - Sesimbra (Terminal)"]],
        204: [["3206", "Carrasqueira - Sesimbra (Terminal)"]],
        205: [["3205", "Cabo Espichel - Sesimbra (Terminal)"]],
        207: [["3721", "Lisboa (Sete Rios) - Sesimbra (Terminal)"]],
        208: [["3635", "Coina (EstaÃ§Ã£o) - Sesimbra (Terminal), via AzeitÃ£o e Sampaio"], ["3641", "Quinta do Conde - Sampaio, via AzeitÃ£o"]],
        210: [["3217", "Azoia - Sampaio"]],
        211: [["3549", "Quinta do Conde - Sesimbra (Terminal), via Sampaio e Marco do Grilo"]],
        219: [["3548", "Marco do Grilo - Quinta do Conde"]],
        221: [["3220", "Sesimbra | Circular"]],
        222: [["3201", "Aldeia do Meco - Sesimbra (Terminal), via Aiana"], ["3202", "Alfarim - Sesimbra (Terminal), via Aiana"], ["3209", "Fornos - Sesimbra (Terminal), via Aiana"]],
        223: [["3208", "Fetais - Sesimbra (Terminal)"]],
        225: [["3640", "Azoia - Vila Nogueira de AzeitÃ£o"]],
        227: [["3212", "MaÃ§Ã£ (Rua Macieira) - Sesimbra (Terminal)"], ["3216", "Alto das Vinhas - Sampaio"]],
        228: [["3218", "Sesimbra (Porto de Abrigo) - Sesimbra (Terminal)"]],
        229: [["3213", "Pinhal de Cima - Sesimbra (Terminal) | Circular"]],
        230: [["4642", "Sesimbra (Terminal) - SetÃºbal (Hospital)"]],
        231: [["3201", "Aldeia do Meco - Sesimbra (Terminal), via Aiana"]],
        234: [["3215", "Fornos - Sampaio"]],
        236: [["3107", "Laranjeiras - Marco do Grilo"]],
        240: [["3210", "Lagoa de Albufeira - Sesimbra (Terminal)"], ["3211", "Lagoa de Albufeira - Sesimbra (Terminal), via Sampaio"]],
        243: [["3207", "Carrasqueira - Sesimbra (Terminal), via Valbom e Sampaio"], ["3221", "Valbom - Sesimbra (Terminal), via Sampaio"]],
        245: [["3650", "Moita - Sesimbra (Terminal)"]],
        247: [["3214", "Sampaio - Santana"]],
        252: [["3720", "Lisboa (Sete Rios) - Quinta do Conde"]],
        254: [["3535", "Cacilhas (Terminal) - Quinta do Conde"], ["3546", "Cruz de Pau - Quinta do Conde"], ["3547", "Fogueteiro (EstaÃ§Ã£o) - Quinta do Conde"]],
        257: [["4560", "Cabanas - Vila Nogueira de AzeitÃ£o"], ["4561", "Cabanas - Vila Nogueira de AzeitÃ£o, via Quinta do PicÃ£o"]],
        259: [["4640", "Casais da Serra - Vila Nogueira de AzeitÃ£o"]],
        260: [["3545", "Corroios (EstaÃ§Ã£o) - Sesimbra (Terminal)"]],
        302: [["4620", "Moita - Paio Pires"]],
        305: [["4610", "Bairro dos Marinheiros - Barreiro (Terminal)"]],
        "307 (Adaptado)": [["4604", "Barreiro (Terminal) - Moita (Escola Fragata do Tejo)"]],
        "311 (Adaptado)": [["4611", "Penalva - Moita (Esc. SecundÃ¡ria)"]],
        "312 (Adaptado)": [["4102", "CabeÃ§o Verde - Sarilhos Pequenos"]],
        "313 (Adaptado)": [["4104", "Moita | Circular"], ["4532", "Moita - Quatro Marcos"]],
        317: [["4602", "Alhos Vedros (EstaÃ§Ã£o) - Barreiro (Terminal)"]],
        318: [["4103", "Moita (EstaÃ§Ã£o) - Sarilhos Pequenos"]],
        "326 (Adaptado)": [["4531", "Moita - Palmela (Terminal)"]],
        330: [["4605", "Lavradio - Pinhal do Forno"]],
        333: [["4701", "Lisboa (Oriente) - Vale da Amoreira"]],
        "336 (Adaptado)": [["4101", "Alhos Vedros (Escola JosÃ© Afonso) - Arroteias"]],
        401: [["4204", "Bairro do CharqueirÃ£o - Montijo (Terminal Fluvial)"], ["4205", "Bairro do CharqueirÃ£o - Montijo (Terminal Fluvial), via Vale Porim"], ["4207", "Montijo (Ã�rea Comercial) - Montijo (Terminal Fluvial)"]],
        403: [["4203", "Afonsoeiro - Montijo (Terminal Fluvial), via Bairro da Liberdade"], ["4206", "Bairro Esteval - Montijo (Terminal Fluvial)"]],
        404: [["4202", "Afonsoeiro - Bairro do Saldanha, via Bairro da CalÃ§ada"]],
        410: [["4511", "Alcochete (Freeport) - Montijo (Terminal RodoviÃ¡rio), via Samouco"], ["4600", "Alcochete (Freeport) - Barreiro (Terminal)"], ["4601", "Barreiro (Terminal) - Montijo (Terminal RodoviÃ¡rio)"], ["4603", "Barreiro (Terminal) - ChÃ£o Duro"]],
        "412 (Adaptado)": [["4501", "Alcochete - Montijo (Terminal Fluvial)"]],
        413: [["4510", "Alcochete (Freeport) - Montijo (Terminal RodoviÃ¡rio)"], ["4512", "Alcochete (Freeport) - SetÃºbal (ITS), via Alto Estanqueiro"], ["4513", "Alcochete (Freeport) - Pinhal Novo"], ["4517", "Montijo (Terminal RodoviÃ¡rio) - SetÃºbal (ITS)"], ["4523", "Montijo (Terminal RodoviÃ¡rio) - Pinhal Novo"]],
        414: [["4514", "Canha - Montijo (Terminal RodoviÃ¡rio), via PegÃµes"]],
        415: [["4504", "Montijo (Terminal Fluvial) - Passil"]],
        416: [["4514", "Canha - Montijo (Terminal RodoviÃ¡rio), via PegÃµes"], ["4515", "Montijo (Terminal RodoviÃ¡rio) - PegÃµes"]],
        419: [["4502", "Alcochete - Passil"]],
        426: [["4516", "Montijo (Terminal RodoviÃ¡rio) - Rio Frio"]],
        431: [["4703", "Lisboa (Oriente) - Montijo (Terminal RodoviÃ¡rio), via Alcochete e Samouco"]],
        432: [["4702", "Lisboa (Oriente) - Valbom"], ["4704", "Atalaia - Lisboa (Oriente)"]],
        435: [["4705", "Lisboa (Oriente) - Samouco"], ["4707", "Lisboa (Oriente) - Montijo (Terminal RodoviÃ¡rio)"]],
        440: [["4643", "Montijo (Av. Inf. D. Henrique) - Sesimbra (Terminal)"]],
        441: [["4210", "Canha - Foros Boavista"], ["4212", "Foros Boavista - PegÃµes"]],
        444: [["4520", "Faias - PegÃµes"]],
        446: [["4521", "Faias - Pinhal Novo"], ["4522", "Faias - PoceirÃ£o"]],
        447: [["4305", "Brejos do Assa - Palmela (Terminal)"], ["4524", "Palmela (Terminal) - PegÃµes"]],
        448: [["4322", "Pinhal Novo - Rio Frio"]],
        449: [["4320", "Pinhal Novo | Circular"], ["4321", "Pinhal Novo - Qta do Anjo"]],
        451: [["4530", "Bairro Vila Morena - Pinhal Novo"]],
        453: [["4201", "Afonsoeiro - Bairro da Liberdade"], ["4706", "SÃ£o Francisco - Lisboa (Oriente)"]],
        454: [["4302", "Palmela (EstaÃ§Ã£o) - Palmela (Terminal)"], ["4307", "Loja Nova - Palmela (Terminal)"]],
        455: [["4311", "Asseiceira - PoceirÃ£o"]],
        561: [["4725", "Lisboa (Sete Rios) - SetÃºbal (ITS)"]],
        562: [["4720", "Lisboa (Oriente) - SetÃºbal (ITS)"]],
        563: [["4715", "Lisboa (Oriente) - SetÃºbal (ITS), via Pinhal Novo"]],
        565: [["4710", "Lisboa (Oriente) - Palmela (Terminal)"], ["4711", "Lisboa (Oriente) - Pinhal Novo"]],
        583: [["3610", "Cacilhas (Terminal) - SetÃºbal (ITS), via A2"]],
        601: [["4406", "Manteigadas - SetÃºbal (Mercado)"], ["4408", "Manteigadas - SetÃºbal (Mercado), via Bela Vista"], ["4412", "Morgada - SetÃºbal (Mercado)"], ["4413", "Morgada - SetÃºbal (Mercado), via Bela Vista"], ["4436", "SetÃºbal (Mercado) - SetÃºbal (Av. Soeiro Pereira Gomes)"]],
        602: [["4416", "PoÃ§o Mouro - SetÃºbal (ITS)"], ["4417", "PoÃ§o Mouro - SetÃºbal (ITS), via Manteigadas"], ["4418", "SetÃºbal (Alegro) - SetÃºbal (Av. 5 Outubro)"], ["4420", "SetÃºbal (Alegro) - SetÃºbal (ITS)"]],
        604: [["4403", "Fonte da Talha - SetÃºbal (Av. LuÃ­sa Todi)"], ["4423", "Amoreiras â€“ SetÃºbal (Av. LuÃ­sa Todi)"], ["4551", "Palmela (USF) - SetÃºbal (Av. LuÃ­sa Todi)"]],
        605: [["4431", "SetÃºbal (ITS) - SetÃºbal (Quinta Varzinha)"]],
        607: [["4434", "SetÃºbal (Mercado 2 de Abril) - SetÃºbal (R. Timor)"]],
        608: [["4424", "SetÃºbal (Bairro Viso) - Manteigadas"], ["4425", "SetÃºbal (Escola Viso) - Mitrena"], ["4426", "SetÃºbal (Bairro Viso) - SetÃºbal (CHEsetÃºbal)"]],
        609: [["4421", "SetÃºbal (Bairro Camolas) - SetÃºbal (Casal Figueiras)"], ["4422", "SetÃºbal (Bairro Camolas) - SetÃºbal (Casal Figueiras), via Bairro do Viso"], ["4428", "SetÃºbal (Casal Figueiras) - Vale Ana Gomes"]],
        610: [["4438", "SetÃºbal (Monte Belo Norte) - SetÃºbal (Saboaria)"], ["4440", "SetÃºbal (Monte Belo Norte) - SetÃºbal (Saboaria), via Alegro"], ["4541", "Algeruz - SetÃºbal (Av. LuÃ­sa Todi)"]],
        612: [["4419", "Brejos Canes - SetÃºbal (Saboaria)"], ["4441", "SetÃºbal (Saboaria) - SetÃºbal (Vale Cobro)"]],
        614: [["4421", "SetÃºbal (Bairro Camolas) - SetÃºbal (Casal Figueiras)"], ["4422", "SetÃºbal (Bairro Camolas) - SetÃºbal (Casal Figueiras), via Bairro do Viso"], ["4433", "Alto Guerra - SetÃºbal (Casal Figueiras)"]],
        616: [["4407", "Manteigadas - SetÃºbal (Mercado), via Bairro da Carmona"]],
        680: [["4301", "Palmela (Centro) - Palmela (Terminal)"]],
        708: [["4545", "Biscainho - SetÃºbal (Bela Vista)"]],
        709: [["4901", "Landeira - SetÃºbal (ITS)"]],
        710: [["4310", "Ã�guas de Moura - PoceirÃ£o"]],
        711: [["4312", "PoceirÃ£o - Vale Abrunheira (X), via Fernando PÃ³"]],
        723: [["4472", "Praia do Creiro - SetÃºbal (ITS)"], ["4476", "Praias ArrÃ¡bida | Circular"]],
        725: [["4474", "Figueirinha - SetÃºbal (Alegro)"]],
        726: [["4471", "Praia Albarquel | Circular"]],
        727: [["4470", "Brejos AzeitÃ£o - Praia do Creiro"]],
        751: [["4414", "OutÃ£o (Hospital) - SetÃºbal (ITS)"], ["4415", "OutÃ£o (Hospital) - SetÃºbal (ITS), via vale da Rasca"]],
        754: [["4631", "Fogueteiro (EstaÃ§Ã£o) - SetÃºbal (ITS)"]],
        755: [["4630", "Corroios (EstaÃ§Ã£o) - SetÃºbal (ITS)"]],
        756: [["4452", "Mitrena (Portucel) - SetÃºbal (ITS)"]],
        757: [["4453", "Mitrena (Portucel) - SetÃºbal (ITS), via Estrada GraÃ§a"]],
        758: [["4548", "LagameÃ§as - SetÃºbal (ITS)"]],
        764: [["4540", "Ã�guas de Moura - SetÃºbal (ITS)"], ["4544", "Bairro MargaÃ§a - SetÃºbal (ITS)"]],
        765: [["4540", "Ã�guas de Moura - SetÃºbal (ITS)"], ["4544", "Bairro MargaÃ§a - SetÃºbal (ITS)"]],
        766: [["4442", "Praias do Sado (EstaÃ§Ã£o) - SetÃºbal (Bela Vista)"]],
        767: [["4547", "Cabanas - SetÃºbal (ITS)"], ["4549", "Palmela (Terminal) - SetÃºbal (ITS)"], ["4562", "SetÃºbal (ITS) - Vila Nogueira de AzeitÃ£o, via Palmela (EstaÃ§Ã£o)"]],
        768: [["4306", "Cabanas - Palmela (Terminal)"], ["4550", "Palmela (Terminal) - Vila Nogueira de AzeitÃ£o"], ["4562", "SetÃºbal (ITS) - Vila Nogueira de AzeitÃ£o, via Palmela (EstaÃ§Ã£o)"]],
        770: [["4641", "Quinta do Conde - SetÃºbal (ITS)"]],
        774: [["4546", "Biscainho - SetÃºbal (ITS)"]],
        776: [["4451", "Mitrena (Lisnave) - SetÃºbal (ITS)"]],
        779: [["4542", "Algeruz - SetÃºbal (ITS)"], ["4543", "Algeruz - SetÃºbal (ITS), via PoÃ§oilos"]],
        780: [["4402", "Estefanilha - SetÃºbal (ITS)"], ["4437", "FaralhÃ£o - SetÃºbal (ITS)"]],
        781: [["4411", "Morgada - SetÃºbal (ITS)"]],
        783: [["3605", "Cacilhas (Terminal) - SetÃºbal (ITS), via AzeitÃ£o"]],
        797: [["4439", "Praias do Sado - SetÃºbal (ITS)"]]
    },
    "Vimeca / Lisboa Transportes": {
        1: [["1101", "Alfragide (Alegro) - AlgÃ©s (Terminal)"], ["1504", "AlgÃ©s (Terminal) - Bairro Zambujal, via Linda-a-Velha"], ["1722", "Alfragide (Alegro) - Hospital SÃ£o Francisco Xavier"]],
        10: [["1503", "AlgÃ©s (Terminal) - Bairro Zambujal"], ["1505", "AlgÃ©s (Terminal) - IKEA Alfragide"]],
        101: [["1717", "Lisboa (C. Militar) - Tercena, via Amadora Este (Metro)"]],
        102: [["1527", "Cruz Quebrada (EstaÃ§Ã£o) - Queluz (EstaÃ§Ã£o)"]],
        103: [["1512", "Amadora (Hospital) - Montelavar"]],
        104: [["1508", "Almargem do Bispo - Amadora Este (Metro)"]],
        105: [["1518", "Monte AbraÃ£o - Reboleira (Metro)"]],
        106: [["1530", "Oeiras (EstaÃ§Ã£o) - Queluz"], ["1601", "Amadora Este (Metro) - Carcavelos (Praia)"], ["1602", "Carcavelos (Praia) - Queluz"]],
        107: [["1716", "Idanha - Lisboa (M. Pombal)"]],
        108: [["1112", "Caxias - Queijas"], ["1507", "Caxias - Reboleira (Metro)"]],
        11: [["1724", "Linda-a-Velha - Lisboa (M. Pombal)"]],
        110: [["1229", "UrbanizaÃ§Ã£o Quinta das Flores (Junta de Freguesia) | Circular"]],
        111: [["1120", "Oeiras (EstaÃ§Ã£o) - PaÃ§o de Arcos (EstaÃ§Ã£o)"]],
        112: [["1211", "Belas - CacÃ©m (EstaÃ§Ã£o)"], ["1523", "CacÃ©m (EstaÃ§Ã£o) - Oeiras (EstaÃ§Ã£o)"]],
        113: [["1714", "Amadora (EstaÃ§Ã£o Sul) - BelÃ©m (EstaÃ§Ã£o)"]],
        114: [["1502", "AlgÃ©s (Terminal) - Amadora (EstaÃ§Ã£o Sul), via Linda-a-Velha"]],
        115: [["1725", "Lisboa (M. Pombal) - Oeiras (EstaÃ§Ã£o)"]],
        116: [["1124", "PaÃ§o de Arcos | Circular"]],
        117: [["1525", "Caxias - Monte AbraÃ£o (EstaÃ§Ã£o)"], ["1526", "Caxias (Est. Prisional) - Monte AbraÃ£o (EstaÃ§Ã£o)"]],
        "118 (Adaptado)": [["1004", "Amadora (EstaÃ§Ã£o Norte) - Moinhos da Funcheira | noturna"], ["1006", "Amadora (EstaÃ§Ã£o Norte) - UBBO | noturna"]],
        119: [["1119", "Leceia - PaÃ§o de Arcos (EstaÃ§Ã£o)"], ["1531", "PaÃ§o de Arcos (EstaÃ§Ã£o) - SÃ£o Marcos"], ["1617", "PaÃ§o de Arcos (EstaÃ§Ã£o) - TalaÃ­de (Igreja)"]],
        12: [["1105", "AlgÃ©s (Terminal) - Queluz Baixo"], ["1108", "Carnaxide (Escola) - Queluz Baixo"], ["1522", "AlgÃ©s (Terminal) - Monte AbraÃ£o (EstaÃ§Ã£o)"]],
        122: [["1529", "Oeiras (EstaÃ§Ã£o) - Bairros dos Navegadores"]],
        124: [["1634", "Fonte da Aranha - Montelavar"]],
        125: [["1610", "PaÃ§o de Arcos (EstaÃ§Ã£o) - TalaÃ­de (Campo de Futebol)"], ["1616", "PaÃ§o de Arcos (EstaÃ§Ã£o) - Taguspark"]],
        126: [["1221", "CacÃ©m (EstaÃ§Ã£o) | Circular"]],
        128: [["1701", "Alto Brandoa - Benfica | madrugada"], ["1702", "Alto Brandoa - Lisboa (C. Militar) | madrugada"], ["1706", "Lisboa (C. Militar) - UBBO"]],
        129: [["1123", "PaÃ§o de Arcos (EstaÃ§Ã£o) - Porto Salvo (Lagoas Park)"]],
        13: [["1726", "Lisboa (M. Pombal) - Queijas"], ["1728", "Lisboa (M. Pombal) - Queijas, via Linda-a-Velha"], ["1730", "Lisboa (M. Pombal) - Queluz Baixo, via Linda-a-Velha"]],
        130: [["1230", "Monte AbraÃ£o (EstaÃ§Ã£o) | Circular"]],
        131: [["1230", "Monte AbraÃ£o (EstaÃ§Ã£o) | Circular"]],
        132: [["1515", "Amadora Este (Metro) - Casal de Cambra"], ["1720", "Casal de Cambra - Lisboa (C. Militar), via Amadora"]],
        133: [["1603", "Amadora (EstaÃ§Ã£o Norte) - CaneÃ§as"]],
        134: [["1517", "Casal de Cambra - Reboleira (Metro)"]],
        "136 (Adaptado)": [["1004", "Amadora (EstaÃ§Ã£o Norte) - Moinhos da Funcheira | noturna"]],
        "137 (Adaptado)": [["1005", "Amadora (EstaÃ§Ã£o Norte) - UBBO"]],
        "13D": [["1727", "Lisboa (M. Pombal) - Queijas, via A5"], ["1729", "Lisboa (M. Pombal) - Queluz Baixo (C.C.)"]],
        140: [["1220", "CacÃ©m (EstaÃ§Ã£o) - SÃ£o Marcos (Largo)"], ["1233", "Mira Sintra (Mercado) - SÃ£o Marcos (Largo)"]],
        142: [["1705", "Benfica (GrÃ£o Vasco) - UBBO, via Amadora Este (Metro) | noturna e madrugada"], ["1707", "Lisboa (C. Militar) - UBBO, via Falagueira"]],
        143: [["1007", "Amadora (EstaÃ§Ã£o Norte) | Circular madrugada"], ["1703", "Amadora (EstaÃ§Ã£o Norte) - Pontinha (Metro)"]],
        144: [["1718", "CacÃ©m (Bairro Grajal) - BelÃ©m (EstaÃ§Ã£o)"]],
        149: [["1715", "BelÃ©m (EstaÃ§Ã£o) - Mira Sintra (Mercado)"]],
        "149/Nova": [["1511", "Amadora (Hospital) - Monte AbraÃ£o (EstaÃ§Ã£o)"]],
        15: [["1727", "Lisboa (M. Pombal) - Queijas, via A5"], ["1732", "Lisboa (M. Pombal) - SÃ£o Marcos, via Carnaxide"], ["1733", "Lisboa (M. Pombal) - SÃ£o Marcos, via Linda-a-Pastora"]],
        150: [["1213", "CacÃ©m (Bairro Joaquim Fontes - Shopping) - CacÃ©m (EstaÃ§Ã£o)"]],
        151: [["1218", "CacÃ©m (EstaÃ§Ã£o) - Mira Sintra (Mercado)"], ["1219", "CacÃ©m (EstaÃ§Ã£o) - Mira Sintra (Mercado), via Av. Santa Maria"]],
        152: [["1204", "Alegro Sintra | Circular"]],
        "154 (Adaptado)": [["1514", "Amadora (Hospital) | Circular, via Brandoa"]],
        "155 (Adaptado)": [["1514", "Amadora (Hospital) | Circular, via Brandoa"]],
        157: [["1519", "Queluz (PalÃ¡cio) - Serra da Silveira"]],
        158: [["1113", "Caxias (EstaÃ§Ã£o) - Lage"], ["1114", "Caxias (EstaÃ§Ã£o) - PaÃ§o de Arcos (EstaÃ§Ã£o)"], ["1115", "Caxias (Pedreira Italiana) - Lage"], ["1116", "Caxias (Pedreira Italiana) - PaÃ§o de Arcos (EstaÃ§Ã£o)"], ["1117", "Caxias (Quinta da Moura) - Lage"], ["1118", "Caxias (Quinta da Moura) - PaÃ§o de Arcos (EstaÃ§Ã£o)"]],
        160: [["1209", "Bairro da Felosa - Mira Sintra (Mercado)"]],
        161: [["1222", "CacÃ©m (EstaÃ§Ã£o) | Circular, via Bairro Joaquim Fontes"]],
        162: [["1713", "AlgÃ©s (Terminal) - Amadora Este (Metro)"]],
        163: [["1227", "MassamÃ¡ (Casal do Olival) - Queluz"], ["1721", "Lisboa (C. Militar) - MassamÃ¡ (Casal do Olival), via Amadora Este (Metro)"]],
        164: [["1224", "Carregueira (Estabelecimento prisional) - Monte AbraÃ£o (EstaÃ§Ã£o)"]],
        165: [["1708", "UBBO | Circular"]],
        "168 (Adaptado)": [["1011", "Brandoa (Largo) - Reboleira (Metro)"]],
        170: [["1215", "CacÃ©m (EstaÃ§Ã£o) - Encosta de SÃ£o Marcos"], ["1217", "CacÃ©m (EstaÃ§Ã£o) - MassamÃ¡ (UrbanizaÃ§Ã£o Norte)"], ["1225", "Encosta de SÃ£o Marcos - MassamÃ¡ (UrbanizaÃ§Ã£o Norte)"]],
        171: [["1528", "Miraflores (Esc. SecundÃ¡ria) - Queluz (EstaÃ§Ã£o)"]],
        179: [["1212", "CacÃ©m (Bairro Grajal) - Queluz (EstaÃ§Ã£o)"]],
        184: [["1611", "PaÃ§o de Arcos (EstaÃ§Ã£o) - TalaÃ­de (Campo de Futebol), via Vila Fria"]],
        185: [["1704", "Amadora (Hospital) - Lisboa (M. Pombal)"]],
        "186 (Adaptado)": [["1510", "Amadora (Hospital) - Damaia (Praceta Liberdade)"]],
        2: [["1103", "AlgÃ©s (Terminal) - Queijas"], ["1107", "AlgÃ©s (Terminal) - Queluz Baixo, via Queijas"]],
        20: [["1712", "AlgÃ©s (Terminal) - Amadora (EstaÃ§Ã£o Sul)"]],
        22: [["1216", "Casal do CotÃ£o | Circular, via Tercena e SÃ£o Marcos"]],
        23: [["1524", "Casal do CotÃ£o | Circular, via Tercena, SÃ£o Marcos e Taguspark"]],
        24: [["1226", "MassamÃ¡ - Queluz (EstaÃ§Ã£o)"]],
        25: [["1513", "Amadora (Hospital) | Circular"]],
        "26 (Adaptado)": [["1003", "Amadora (EstaÃ§Ã£o Norte) - Amadora Este (Metro)"]],
        6: [["1104", "AlgÃ©s (Terminal) - Queijas, via Jamor"], ["1106", "AlgÃ©s (Terminal) - Queluz Baixo, via Jamor"]],
        "7/13": [["1723", "Carnaxide - Lisboa (M. Pombal)"]]
    }
};
var ficheiro;

function mostraHora() {
    var a = new Date;
    a.toLocaleDateString("pt-PT", {weekday: "long", year: "numeric", month: "long", day: "numeric"});
    var o = a.getFullYear(), s = ("0" + (a.getMonth() + 1)).slice(-2), i = ("0" + a.getDate()).slice(-2),
        r = "" + a.getHours(), e = ("00" + a.getMinutes()).slice(-2), n = parseInt(`${o}${s}${i}`, 10);
    hora_agora = parseInt(`${r}${e}`, 10);
    return document.querySelector("#date").innerHTML = `<b>${r}h${e}</b>,  ${i} de ${["Janeiro", "Fevereiro", "MarÃ§o", "Abril", "Maio", "Junho", "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"][s - 1]} de ${o}`, [n, hora_agora]
}

function mostraHoraEDiaNosInputs() {
    var a = (new Date).toLocaleDateString("pt-PT").replaceAll("/", "-"),
        o = (new Date).toLocaleTimeString("pt-PT").substring(0, 5), s = a.substring(6), i = a.substring(3, 5),
        r = a.substring(0, 2);
    return a = s + "-" + i + "-" + r, document.querySelector("#timeInput").value = o, document.querySelector("#dateInput").value = a, [parseInt(a.replaceAll("-", ""), 10), parseInt(o.replace(":", ""), 10)]
}

function getHoraDiaInserida() {
    var a = document.querySelector("#dateInput").value, o = document.querySelector("#timeInput").value;
    hora_agora = parseInt(o.replace(":", ""), 10), today = parseInt(a.replaceAll("-", ""), 10), document.querySelector("#paragens").innerHTML = "", document.querySelector("#variante").innerHTML = "", mostraLinha()
}

function mostraSelectLinhas() {
    var a = document.querySelector("#selectLinha");
    const o = document.createElement("option");
    let s = document.createTextNode("linha");
    o.setAttribute("value", "linha"), o.appendChild(s), a.appendChild(o);
    for (const [o, s] of Object.entries(designacao_linhas)) {
        const s = document.createElement("option");
        let i = document.createTextNode(`${o}`);
        s.setAttribute("value", o), s.appendChild(i), a.appendChild(s)
    }
}

function mostraLinha() {
    var a = document.querySelector("#selectLinha").value;
    if ("linha" == a) return document.querySelector("#paragens").innerHTML = "", document.querySelector("#variante").innerHTML = "", document.querySelector("#inverterDirecao").style.visibility = "hidden", 0;
    var o = document.querySelector("#direcao").innerHTML, s = linhas_e_seus_horarios[a][o].length,
        i = linhas_e_seus_horarios[a].ida.length, r = linhas_e_seus_horarios[a].volta.length,
        e = linhas_e_seus_horarios[a].circular.length;
    if (document.querySelector("#inverterDirecao").style.visibility = i + r + e == 1 || i >= 1 && 0 == e && 0 == r || r >= 1 && 0 == e && 0 == i ? "hidden" : "visible", i + r + e == 1) i ? (criaHorario(linhas_e_seus_horarios[a].ida[0]), document.querySelector("#variante").innerHTML = designacao_linhas[a].ida) : r ? (criaHorario(linhas_e_seus_horarios[a].volta[0]), document.querySelector("#variante").innerHTML = designacao_linhas[a].volta) : (criaHorario(linhas_e_seus_horarios[a].circular[0]), document.querySelector("#variante").innerHTML = designacao_linhas[a].circular); else if (1 == s && 0 == e) criaHorario(linhas_e_seus_horarios[a][o][0]), document.querySelector("#variante").innerHTML = designacao_linhas[a][o]; else {
        var n = document.createElement("select");
        n.setAttribute("name", "horario"), n.setAttribute("id", "selectVariante"), variante = 1, linhas_e_seus_horarios[a][o].forEach((function (s) {
            const i = document.createElement("option");
            i.setAttribute("value", s), i.style.display = "flex", i.style.flexDirection = "column", i.style.justifyContent = "center", i.querySelectorAll("span").forEach((a => {
                a.style.display = "inline-block"
            })), i.innerHTML = `${designacao_linhas[a][o + "_short"]}`;
            var r = i.querySelector("span.ultimo").innerHTML + " " + variante;
            i.querySelector("span.ultimo").innerHTML = r, variante++, i.querySelectorAll("span").forEach((a => {
                a.style.display = "inline-block"
            })), n.appendChild(i)
        })), linhas_e_seus_horarios[a].circular.forEach((function (s) {
            const i = document.createElement("option");
            i.setAttribute("value", s), i.style.display = "inline-block";
            let r = document.createTextNode(`${designacao_linhas[a][o + "_short"]}`);
            i.appendChild(r), n.appendChild(i)
        })), document.querySelector("#variante").innerHTML = "";
        document.querySelector("#variante").appendChild(n), document.querySelectorAll("span.long").forEach((a => {
            a.style.display = "none"
        })), document.querySelector("#selectVariante").onchange = () => {
            criaHorario(document.querySelector("#selectVariante").value)
        }
    }
}

function criaHorario(a) {
    document.querySelector("#meuHorarioEscolhas").innerHTML = "", document.querySelector("#paragens").style.display = "block", document.querySelector("#comandosControlo").style.display = "grid";
    const o = document.querySelector("#tipoHorario").innerHTML;
    "proximo" == o ? (document.querySelector("#horaEDia").style.visibility = "visible", criaHorarioProximo(a)) : "completo" == o ? (document.querySelector("#horaEDia").style.visibility = "visible", meuHorarioEscolhas.innerHTML = "", criaHorarioCompleto(a, !1, "horario_do_dia")) : (document.querySelector("#horaEDia").style.visibility = "hidden", criaMeuHorario(a))
}

function criaMeuHorario(a) {
    document.querySelector("#paragens").innerHTML = "";
    const o = document.querySelector("#meuHorarioEscolhas");
    var s = new Map;
    fetch("images/horarios/" + a).then((a => a.json())).then((a => {
        var i = new Set;
        for (let o = 0; o < a[0][1].length; o++) i.add(a[0][1][o][1]);
        Array.from(i).sort().forEach((a => {
            s.set(a, service_id[a])
        })), o.innerHTML = "", table = td = document.createElement("table"), o.append(table);
        var r = 0;
        s.forEach(((a, o) => {
            var s = document.createElement("tr");
            const i = document.createElement("input");
            i.type = "checkbox", i.name = o, i.id = "plano_" + o, i.value = o, i.setAttribute("id", "td" + ++r), td = document.createElement("td"), td.appendChild(i), s.appendChild(td), label = document.createElement("label"), label.setAttribute("for", `td${r}`), label.innerHTML = `<span class="service_id">${o}</span>`, label.setAttribute("for", `td${r}`), td = document.createElement("td"), td.appendChild(label), s.appendChild(td), label = document.createElement("label"), label.innerHTML = `${a}`, label.setAttribute("for", `td${r}`), td = document.createElement("td"), td.appendChild(label), s.appendChild(td), table.appendChild(s)
        }))
    })).then((() => {
        document.querySelectorAll("#meuHorarioEscolhas input").forEach((o => {
            o.onchange = () => {
                var o = new Set;
                document.querySelectorAll("#meuHorarioEscolhas input").forEach((a => {
                    a.checked && o.add(a.value)
                })), service_sub_list = Array.from(o), criaHorarioCompleto(a, !1, service_sub_list)
            }
        }))
    }))
}

function abrePrimeiraParagem() {
    paragem = document.querySelector("#paragens .paragem:first-child"), paragem.style.backgroundColor = "whitesmoke";
    let a = paragem.querySelector(".horario");
    paragem.querySelector(".nome").style.fontWeight = "bold", paragem.style.backgroundColor = "whitesmoke", a.style.display = "block", paragem.style.border = "1px solid black"
}

function criaHorarioProximo(a) {
    const o = document.createElement("div");
    o.setAttribute("id", "linhaVerticalAmarela"), document.querySelector("#paragens").appendChild(o), meuHorarioEscolhas.innerHTML = "", fetch("images/horarios/" + a).then((a => a.json())).then((a => {
        var o = 0;
        document.getElementById("paragens").innerHTML = "", a.forEach((a => {
            const s = document.createElement("div");
            s.setAttribute("class", "paragem"), s.setAttribute("data-paragem", "" + o++);
            const i = document.createElement("div");
            i.setAttribute("class", "bola"), i.innerHTML = "âš«", s.appendChild(i);
            const r = document.createElement("div");
            r.setAttribute("class", "nome");
            const e = document.createTextNode(a[0]);
            r.appendChild(e), s.appendChild(r);
            var n = [];
            for (let o = 0; o < a[1].length; o++) {
                var l = parseInt(a[1][o][0].replace(":", ""));
                a[1][o][1] in date_service_ids[today] && l > hora_agora && n.push(a[1][o][0])
            }
            const t = document.createElement("div");
            t.setAttribute("class", "horario"), t.style.display = "none";
            const d = document.createElement("span");
            var p;
            d.setAttribute("class", "proxima"), p = n.length > 0 ? document.createTextNode("PrÃ³xima: " + n[0]) : document.createTextNode("hoje nÃ£o existem mais carreiras"), d.appendChild(p), t.appendChild(d);
            const c = document.createElement("span");
            c.setAttribute("class", "seguintes"), c.style.padding = "0 0 0 1em";
            var u = document.createTextNode("");
            1 == n.length ? u = document.createTextNode("(Ãºltima)") : 2 == n.length ? u = document.createTextNode(`Seguinte: ${n[1]}`) : n.length > 2 && (u = document.createTextNode(`Seguintes: ${n[1]}, ${n[2]}`)), c.appendChild(u), t.appendChild(c), s.appendChild(t), document.getElementById("paragens").appendChild(s)
        })), abrePrimeiraParagem()
    })).then((() => {
        let a = document.createElement("div");
        a.setAttribute("id", "linhaPreta"), a.style.padding = "0 5px", a.style.backgroundColor = "#ffdd00", a.style.height = "100%", a.style.position = "absolute", a.style.zIndex = "1", a.style.left = "1em", a.style.top = "0", document.getElementById("paragens").appendChild(a)
    })).then((() => {
        document.querySelectorAll(".paragem").forEach((a => {
            a.onmouseover = () => {
                a.style.backgroundColor = "whitesmoke"
            }, a.onmouseout = () => {
                "1px solid black" != a.style.border && (a.style.backgroundColor = "")
            }
        }))
    })).then((() => {
        document.querySelectorAll(".paragem").forEach((a => {
            a.onclick = () => {
                a.style.backgroundColor = "whitesmoke";
                let o = a.querySelector(".horario");
                if ("block" != o.style.display) var s = !0;
                document.querySelectorAll(".paragem").forEach((a => {
                    a.style.backgroundColor = "", a.style.border = "none", a.querySelector(".horario").style.display = "none", a.querySelector(".nome").style.fontWeight = "normal"
                })), s && (a.querySelector(".nome").style.fontWeight = "bold", a.style.backgroundColor = "whitesmoke", o.style.display = "block", a.style.border = "1px solid black")
            }
        }))
    }))
}

function criaHorarioCompleto(a, o, s) {
    if (0 == s.length) return divParagens = document.getElementById("paragens"), void (divParagens.innerHTML = "");
    fetch("images/horarios/" + a).then((a => a.json())).then((a => {
        divParagens = document.getElementById("paragens"), divParagens.innerHTML = "";
        const i = document.createElement("div");
        i.setAttribute("class", "div_da_tabela"), divParagens.appendChild(i);
        const r = document.createElement("table");
        if (r.setAttribute("class", "tabela"), i.appendChild(r), service_id_map = new Map, "horario_do_dia" != s) {
            paragemRow = document.createElement("tr"), paragemRow.setAttribute("class", "paragemRow"), nomeTd = document.createElement("td"), nomeTd.setAttribute("class", "nome"), nome = document.createTextNode("Dias de operaÃ§Ã£o"), nomeTd.appendChild(nome), nomeTd.setAttribute("style", "background-color:#ffdd00"), paragemRow.appendChild(nomeTd);
            for (let i = 0; i < a[0][1].length; i++) {
                let r = a[0][1][i][1];
                if (1 == o || s.includes(r.toString())) {
                    let a = document.createElement("td");
                    a.setAttribute("class", "tipoLinha"), a.innerHTML = `<span class="service_id">${r}</span>`;
                    let o = document.createElement("div");
                    o.setAttribute("class", "descricao"), o.innerHTML = service_id[r], a.appendChild(o), paragemRow.appendChild(a), service_id_map.set(r, service_id[r])
                }
            }
            r.appendChild(paragemRow)
        }
        a.forEach((a => {
            paragemRow = document.createElement("tr"), paragemRow.setAttribute("class", "paragemRow");
            const i = document.createElement("td");
            i.setAttribute("class", "nome");
            const e = document.createTextNode(a[0]);
            i.appendChild(e), paragemRow.appendChild(i);
            for (let i = 0; i < a[1].length; i++) if (1 == o || "horario_do_dia" == s && date_service_ids[today].includes(`${a[1][i][1]}`) || s.includes(`${a[1][i][1]}`)) {
                let o = document.createElement("td");
                o.setAttribute("class", "hora"), o.innerHTML = a[1][i][0], paragemRow.appendChild(o)
            }
            r.appendChild(paragemRow)
        })), 1 != o && "horario_do_dia" == s || (paragemRow = document.createElement("div"), paragemRow.setAttribute("class", "infoRow"), titulo = document.createElement("div"), titulo.innerHTML = "HorÃ¡rio vÃ¡lido para os seguintes dias de operaÃ§Ã£o:", paragemRow.appendChild(titulo), divParagens.appendChild(paragemRow), listaInfo = document.createElement("ul"), listaInfo.setAttribute("class", "infoUl"), service_id_map.forEach(((a, i) => {
            (1 == o || s.includes(i.toString())) && (ulLinha = document.createElement("li"), ulLinha.setAttribute("class", "infoLi"), ulLinha.innerHTML = `${i}: ${a}`, listaInfo.appendChild(ulLinha))
        })), paragemRow.appendChild(listaInfo), divParagens.appendChild(paragemRow))
    }))
}

function mostraBotoes() {
    document.querySelectorAll("#comandosControlo button").forEach((a => {
        a.onclick = () => {
            switch (document.querySelectorAll("#comandosControlo button").forEach((a => {
                a.style.backgroundColor = "whitesmoke", a.style.color = "grey"
            })), a.style.backgroundColor = "#ffdd00", a.style.color = "black", a.dataset.tipo) {
                case"proximo":
                    document.querySelector("#tipoHorario").innerHTML = "proximo", document.querySelector("#save").style.visibility = "hidden";
                    break;
                case"completo":
                    document.querySelector("#tipoHorario").innerHTML = "completo", document.querySelector("#save").style.visibility = "visible";
                    break;
                case"meuHorario":
                    document.querySelector("#tipoHorario").innerHTML = "meuHorario", document.querySelector("#save").style.visibility = "visible"
            }
            mostraLinha()
        }
    })), document.querySelector("#comandosControlo").style.display = "none"
}

function printFunction() {
    const a = document.querySelector("#toprint");
    a.innerHTML = "", logo = document.createElement("img"), logo.setAttribute("src", "images/logo.png");
    const o = document.createElement("div");
    o.setAttribute("class", "headerPrint"), o.innerHTML = "<img src='images/logo.png'>", o.innerHTML += "<h3><br>" + document.querySelector("#linha").innerHTML + "</h3>";
    "completo" == document.querySelector("#tipoHorario").innerHTML ? o.innerHTML += ' <p class="infoHeaderPrint">  HorÃ¡rio vÃ¡lido em: ' + document.querySelector("#dateInput").value + "</p><br><br>" : o.appendChild(document.querySelector(".infoRow")), a.appendChild(o);
    const s = document.createElement("div");
    s.innerHTML = document.querySelector("#paragens").innerHTML, a.appendChild(s), window.print()
}

document.addEventListener("DOMContentLoaded", (() => {
    [today, hora_agora] = mostraHoraEDiaNosInputs(), mostraSelectLinhas(), document.querySelector("#tipoHorario").innerHTML = "proximo", document.querySelector("#proximosButton").style.backgroundColor = "#ffdd00", document.querySelector("#proximosButton").style.color = "black", document.querySelector("#save").style.visibility = "hidden", mostraBotoes(), document.querySelector("#inverterDirecao").style.visibility = "hidden", document.querySelector("#selectLinha").onchange = mostraLinha, document.querySelector("#inverterDirecao").onclick = () => {
        "ida" == document.querySelector("#direcao").innerHTML ? document.querySelector("#direcao").innerHTML = "volta" : document.querySelector("#direcao").innerHTML = "ida", mostraLinha()
    }
}));
const date_service_ids = {
        20220101: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
        20220102: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20220103: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220104: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220105: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220106: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220107: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220108: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220109: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
        20220110: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220111: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220112: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220113: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220114: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220115: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220116: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220117: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220118: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220119: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220120: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220121: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220122: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220123: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220124: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220125: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220126: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220127: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220128: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220129: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220130: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220131: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220201: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220202: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220203: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220204: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220205: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220206: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20220207: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220208: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220209: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220210: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220211: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220212: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220213: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
        20220214: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220215: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220216: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220217: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220218: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220219: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220220: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220221: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220222: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220223: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220224: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220225: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220226: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220227: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220228: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220301: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220302: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220303: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220304: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220305: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220306: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20220307: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220308: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220309: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220310: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220311: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220312: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220313: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
        20220314: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220315: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220316: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220317: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220318: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220319: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220320: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220321: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220322: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220323: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220324: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220325: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220326: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220327: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220328: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220329: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220330: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220331: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220401: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220402: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220403: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20220404: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220405: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220406: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220407: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220408: ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20220409: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
        20220410: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
        20220411: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20220412: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20220413: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20220414: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20220415: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
        20220416: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
        20220417: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220418: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20220419: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20220420: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20220421: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20220422: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220423: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220424: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220425: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
        20220426: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220427: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220428: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220429: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220430: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220501: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20220502: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220503: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220504: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220505: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220506: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220507: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220508: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
        20220509: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220510: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220511: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220512: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220513: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220514: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220515: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220516: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220517: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220518: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220519: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220520: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220521: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220522: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220523: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220524: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220525: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220526: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220527: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220528: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220529: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220530: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220531: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220601: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220602: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220603: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220604: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220605: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20220606: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220607: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220608: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220609: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220610: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220611: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115", "118", "119", "120", "121"],
        20220612: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220613: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
        20220614: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
        20220615: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
        20220616: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220617: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
        20220618: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115", "118", "119", "120", "121"],
        20220619: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220620: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
        20220621: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
        20220622: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
        20220623: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20220624: ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20220625: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220626: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220627: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20220628: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20220629: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20220630: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20220701: ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "121"],
        20220702: ["1", "3", "7", "8", "11", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220703: ["1", "2", "5", "8", "54", "56", "97", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220704: ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220705: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220706: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220707: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220708: ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220709: ["1", "3", "7", "8", "11", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220710: ["1", "2", "5", "8", "54", "56", "69", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220711: ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220712: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220713: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220714: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220715: ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220716: ["1", "3", "7", "8", "11", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220717: ["1", "2", "5", "8", "54", "56", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220718: ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220719: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220720: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220721: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220722: ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220723: ["1", "3", "7", "8", "11", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220724: ["1", "2", "5", "8", "54", "56", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220725: ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220726: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220727: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220728: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220729: ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "121"],
        20220730: ["1", "3", "7", "8", "11", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220731: ["1", "2", "5", "8", "54", "56", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220801: ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220802: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220803: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220804: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220805: ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220806: ["1", "3", "7", "8", "11", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220807: ["1", "2", "5", "8", "54", "56", "97", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220808: ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220809: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220810: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220811: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220812: ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220813: ["1", "3", "7", "8", "11", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220814: ["1", "2", "5", "8", "54", "56", "69", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220815: ["1", "2", "5", "8", "54", "56", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220816: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220817: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220818: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220819: ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220820: ["1", "3", "7", "8", "11", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220821: ["1", "2", "5", "8", "54", "56", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220822: ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220823: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220824: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220825: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220826: ["1", "2", "3", "4", "11", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220827: ["1", "3", "7", "8", "11", "54", "56", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220828: ["1", "2", "5", "8", "54", "56", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220829: ["1", "2", "3", "4", "41", "51", "56", "60", "66", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220830: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220831: ["1", "2", "3", "4", "41", "51", "56", "60", "69", "77", "102", "109", "111", "113", "118", "119", "120", "121"],
        20220901: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
        20220902: ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
        20220903: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220904: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220905: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
        20220906: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
        20220907: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
        20220908: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
        20220909: ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
        20220910: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220911: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20220912: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
        20220913: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
        20220914: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
        20220915: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118", "119"],
        20220916: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220917: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220918: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220919: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220920: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220921: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220922: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220923: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220924: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20220925: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20220926: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220927: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220928: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220929: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20220930: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221001: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20221002: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20221003: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221004: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221005: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
        20221006: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221007: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221008: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20221009: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
        20221010: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221011: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221012: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221013: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221014: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221015: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20221016: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20221017: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221018: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221019: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221020: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221021: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221022: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20221023: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20221024: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221025: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221026: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221027: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221028: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221029: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20221030: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20221031: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221101: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
        20221102: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221103: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221104: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221105: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20221106: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20221107: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221108: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221109: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221110: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221111: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221112: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20221113: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
        20221114: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221115: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221116: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221117: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221118: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221119: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20221120: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20221121: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221122: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221123: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221124: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221125: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221126: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20221127: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20221128: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221129: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221130: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221201: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
        20221202: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221203: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20221204: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20221205: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221206: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221207: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221208: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
        20221209: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221210: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20221211: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
        20221212: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221213: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221214: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221215: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221216: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20221217: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
        20221218: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20221219: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20221220: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20221221: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20221222: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20221223: ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20221224: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
        20221225: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20221226: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20221227: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20221228: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20221229: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20221230: ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20221231: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
        20230101: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20230102: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20230103: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230104: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230105: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230106: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230107: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230108: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
        20230109: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230110: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230111: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230112: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230113: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230114: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230115: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20230116: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230117: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230118: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230119: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230120: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230121: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230122: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20230123: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230124: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230125: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230126: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230127: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230128: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230129: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20230130: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230131: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230201: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230202: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230203: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230204: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230205: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20230206: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230207: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230208: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230209: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230210: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230211: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230212: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
        20230213: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230214: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230215: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230216: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230217: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230218: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
        20230219: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20230220: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20230221: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
        20230222: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20230223: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230224: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230225: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230226: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20230227: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230228: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230301: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230302: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230303: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230304: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230305: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20230306: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230307: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230308: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230309: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230310: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230311: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230312: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
        20230313: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230314: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230315: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230316: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230317: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230318: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230319: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20230320: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230321: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230322: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230323: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230324: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230325: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
        20230326: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20230327: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20230328: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20230329: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20230330: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20230331: ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20230401: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
        20230402: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20230403: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20230404: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20230405: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20230406: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113"],
        20230407: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
        20230408: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115"],
        20230409: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
        20230410: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230411: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230412: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230413: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230414: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230415: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230416: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20230417: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230418: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230419: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230420: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230421: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230422: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230423: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20230424: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230425: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
        20230426: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230427: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230428: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230429: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230430: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20230501: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
        20230502: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230503: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230504: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230505: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230506: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230507: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20230508: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230509: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230510: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230511: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230512: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230513: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230514: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115"],
        20230515: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230516: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230517: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230518: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230519: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230520: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230521: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20230522: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230523: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230524: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230525: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230526: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230527: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230528: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115"],
        20230529: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230530: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230531: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230601: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230602: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230603: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115"],
        20230604: ["1", "2", "5", "8", "39", "40", "97", "109", "111", "112", "113", "115"],
        20230605: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230606: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230607: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230608: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115"],
        20230609: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115"],
        20230610: ["1", "2", "5", "8", "39", "40", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20230611: ["1", "2", "5", "8", "39", "40", "69", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20230612: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
        20230613: ["1", "2", "3", "4", "15", "25", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
        20230614: ["1", "2", "3", "4", "15", "20", "21", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
        20230615: ["1", "2", "3", "4", "15", "20", "28", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
        20230616: ["1", "2", "3", "4", "11", "15", "20", "28", "30", "36", "40", "62", "63", "66", "69", "77", "100", "112", "115", "118"],
        20230617: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "63", "69", "100", "102", "109", "111", "112", "115", "118", "119", "120", "121"],
        20230618: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20230619: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20230620: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20230621: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20230622: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20230623: ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20230624: ["1", "3", "7", "8", "11", "38", "39", "40", "60", "62", "63", "69", "77", "100", "102", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20230625: ["1", "2", "5", "8", "39", "40", "87", "109", "111", "112", "113", "115", "118", "119", "120", "121"],
        20230626: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20230627: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20230628: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20230629: ["1", "2", "3", "4", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"],
        20230630: ["1", "2", "3", "4", "11", "36", "40", "41", "50", "60", "69", "77", "109", "112", "113", "118"]
    }, designacao_linhas = {
        4001: {
            ida: "",
            volta: "",
            circular: "Alcochete | Circular",
            ida_short: "",
            volta_short: "",
            circular_short: "Alcochete | Circ"
        },
        4002: {
            ida: "",
            volta: "",
            circular: "SÃ£o Francisco | Circular",
            ida_short: "",
            volta_short: "",
            circular_short: "SÃ£o Francisco | "
        },
        4103: {
            ida: "<span>Moita (EstaÃ§Ã£o) </span><span>ğŸ –</span><span class='ultimo'> Sarilhos Pequenos</span>",
            volta: "<span> Sarilhos Pequenos</span><span>ğŸ –</span><span class='ultimo'>Moita (EstaÃ§Ã£o) </span>",
            circular: "",
            ida_short: "<span>Moita (E</span><span>ğŸ –</span><span class='ultimo'> Sarilho</span>",
            volta_short: "<span> Sarilho</span><span>ğŸ –</span><span class='ultimo'>Moita (E</span>",
            circular_short: ""
        },
        4101: {
            ida: "<span>Alhos Vedros (Escola JosÃ© Afonso) </span><span>ğŸ –</span><span class='ultimo'> Arroteias</span>",
            volta: "<span> Arroteias</span><span>ğŸ –</span><span class='ultimo'>Alhos Vedros (Escola JosÃ© Afonso) </span>",
            circular: "",
            ida_short: "<span>Alhos Ve</span><span>ğŸ –</span><span class='ultimo'> Arrotei</span>",
            volta_short: "<span> Arrotei</span><span>ğŸ –</span><span class='ultimo'>Alhos Ve</span>",
            circular_short: ""
        },
        4104: {
            ida: "",
            volta: "",
            circular: "Moita | Circular",
            ida_short: "",
            volta_short: "",
            circular_short: "Moita | Circular"
        },
        4205: {
            ida: "<span>Bairro do CharqueirÃ£o </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal Fluvial), via Vale Porim</span>",
            volta: "<span> Montijo (Terminal Fluvial), via Vale Porim</span><span>ğŸ –</span><span class='ultimo'>Bairro do CharqueirÃ£o </span>",
            circular: "",
            ida_short: "<span>Bairro d</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
            volta_short: "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Bairro d</span>",
            circular_short: ""
        },
        4204: {
            ida: "<span>Bairro do CharqueirÃ£o </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal Fluvial)</span>",
            volta: "<span> Montijo (Terminal Fluvial)</span><span>ğŸ –</span><span class='ultimo'>Bairro do CharqueirÃ£o </span>",
            circular: "",
            ida_short: "<span>Bairro d</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
            volta_short: "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Bairro d</span>",
            circular_short: ""
        },
        4206: {
            ida: "<span>Bairro Esteval </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal Fluvial)</span>",
            volta: "<span> Montijo (Terminal Fluvial)</span><span>ğŸ –</span><span class='ultimo'>Bairro Esteval </span>",
            circular: "",
            ida_short: "<span>Bairro E</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
            volta_short: "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Bairro E</span>",
            circular_short: ""
        },
        4203: {
            ida: "<span>Afonsoeiro </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal Fluvial), via Bairro da Liberdade</span>",
            volta: "<span> Montijo (Terminal Fluvial), via Bairro da Liberdade</span><span>ğŸ –</span><span class='ultimo'>Afonsoeiro </span>",
            circular: "",
            ida_short: "<span>Afonsoei</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
            volta_short: "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Afonsoei</span>",
            circular_short: ""
        },
        4202: {
            ida: "<span>Afonsoeiro </span><span>ğŸ –</span><span class='ultimo'> Bairro do Saldanha, via Bairro da CalÃ§ada</span>",
            volta: "<span> Bairro do Saldanha, via Bairro da CalÃ§ada</span><span>ğŸ –</span><span class='ultimo'>Afonsoeiro </span>",
            circular: "",
            ida_short: "<span>Afonsoei</span><span>ğŸ –</span><span class='ultimo'> Bairro </span>",
            volta_short: "<span> Bairro </span><span>ğŸ –</span><span class='ultimo'>Afonsoei</span>",
            circular_short: ""
        },
        4208: {
            ida: "<span>Montijo (Terminal RodoviÃ¡rio) </span><span>ğŸ –</span><span class='ultimo'> Sarilhos Grandes (Estr. 4 Marcos)</span>",
            volta: "<span> Sarilhos Grandes (Estr. 4 Marcos)</span><span>ğŸ –</span><span class='ultimo'>Montijo (Terminal RodoviÃ¡rio) </span>",
            circular: "",
            ida_short: "<span>Montijo </span><span>ğŸ –</span><span class='ultimo'> Sarilho</span>",
            volta_short: "<span> Sarilho</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
            circular_short: ""
        },
        4212: {
            ida: "<span>Foros Boavista </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
            volta: "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Foros Boavista </span>",
            circular: "",
            ida_short: "<span>Foros Bo</span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
            volta_short: "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Foros Bo</span>",
            circular_short: ""
        },
        4210: {
            ida: "<span>Canha </span><span>ğŸ –</span><span class='ultimo'> Foros Boavista</span>",
            volta: "<span> Foros Boavista</span><span>ğŸ –</span><span class='ultimo'>Canha </span>",
            circular: "",
            ida_short: "<span>Canha </span><span>ğŸ –</span><span class='ultimo'> Foros B</span>",
            volta_short: "<span> Foros B</span><span>ğŸ –</span><span class='ultimo'>Canha </span>",
            circular_short: ""
        },
        4201: {
            ida: "<span>Afonsoeiro </span><span>ğŸ –</span><span class='ultimo'> Bairro da Liberdade</span>",
            volta: "<span> Bairro da Liberdade</span><span>ğŸ –</span><span class='ultimo'>Afonsoeiro </span>",
            circular: "",
            ida_short: "<span>Afonsoei</span><span>ğŸ –</span><span class='ultimo'> Bairro </span>",
            volta_short: "<span> Bairro </span><span>ğŸ –</span><span class='ultimo'>Afonsoei</span>",
            circular_short: ""
        },
        4211: {
            ida: "",
            volta: "",
            circular: "Craveiras - PegÃµes | Circular",
            ida_short: "",
            volta_short: "",
            circular_short: "Craveiras - PegÃµ"
        },
        4207: {
            ida: "<span>Montijo (Ã�rea Comercial) </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal Fluvial)</span>",
            volta: "<span> Montijo (Terminal Fluvial)</span><span>ğŸ –</span><span class='ultimo'>Montijo (Ã�rea Comercial) </span>",
            circular: "",
            ida_short: "<span>Montijo </span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
            volta_short: "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
            circular_short: ""
        },
        4303: {
            ida: "",
            volta: "",
            circular: "Palmela | Circular",
            ida_short: "",
            volta_short: "",
            circular_short: "Palmela | Circul"
        },
        4305: {
            ida: "<span>Brejos do Assa </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
            volta: "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Brejos do Assa </span>",
            circular: "",
            ida_short: "<span>Brejos d</span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
            volta_short: "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Brejos d</span>",
            circular_short: ""
        },
        4304: {
            ida: "<span>Palmela (Terminal) </span><span>ğŸ –</span><span class='ultimo'> Penalva</span>",
            volta: "<span> Penalva</span><span>ğŸ –</span><span class='ultimo'>Palmela (Terminal) </span>",
            circular: "",
            ida_short: "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> Penalva</span>",
            volta_short: "<span> Penalva</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
            circular_short: ""
        },
        4322: {
            ida: "<span>Pinhal Novo </span><span>ğŸ –</span><span class='ultimo'> Rio Frio</span>",
            volta: "<span> Rio Frio</span><span>ğŸ –</span><span class='ultimo'>Pinhal Novo </span>",
            circular: "",
            ida_short: "<span>Pinhal N</span><span>ğŸ –</span><span class='ultimo'> Rio Fri</span>",
            volta_short: "<span> Rio Fri</span><span>ğŸ –</span><span class='ultimo'>Pinhal N</span>",
            circular_short: ""
        },
        4307: {
            ida: "<span>Loja Nova </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
            volta: "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Loja Nova </span>",
            circular: "",
            ida_short: "<span>Loja Nov</span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
            volta_short: "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Loja Nov</span>",
            circular_short: ""
        },
        4302: {
            ida: "<span>Palmela (EstaÃ§Ã£o) </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
            volta: "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Palmela (EstaÃ§Ã£o) </span>",
            circular: "",
            ida_short: "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
            volta_short: "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
            circular_short: ""
        },
        4311: {
            ida: "<span>Asseiceira </span><span>ğŸ –</span><span class='ultimo'> PoceirÃ£o</span>",
            volta: "<span> PoceirÃ£o</span><span>ğŸ –</span><span class='ultimo'>Asseiceira </span>",
            circular: "",
            ida_short: "<span>Asseicei</span><span>ğŸ –</span><span class='ultimo'> PoceirÃ£</span>",
            volta_short: "<span> PoceirÃ£</span><span>ğŸ –</span><span class='ultimo'>Asseicei</span>",
            circular_short: ""
        },
        4301: {
            ida: "<span>Palmela (Centro) </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
            volta: "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Palmela (Centro) </span>",
            circular: "",
            ida_short: "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
            volta_short: "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
            circular_short: ""
        },
        4310: {
            ida: "<span>Ã�guas de Moura </span><span>ğŸ –</span><span class='ultimo'> PoceirÃ£o</span>",
            volta: "<span> PoceirÃ£o</span><span>ğŸ –</span><span class='ultimo'>Ã�guas de Moura </span>",
            circular: "",
            ida_short: "<span>Ã�guas de</span><span>ğŸ –</span><span class='ultimo'> PoceirÃ£</span>",
            volta_short: "<span> PoceirÃ£</span><span>ğŸ –</span><span class='ultimo'>Ã�guas de</span>",
            circular_short: ""
        },
        4312: {
            ida: "<span>PoceirÃ£o </span><span>ğŸ –</span><span class='ultimo'> Vale Abrunheira (X), via Fernando PÃ³</span>",
            volta: "<span> Vale Abrunheira (X), via Fernando PÃ³</span><span>ğŸ –</span><span class='ultimo'>PoceirÃ£o </span>",
            circular: "",
            ida_short: "<span>PoceirÃ£o</span><span>ğŸ –</span><span class='ultimo'> Vale Ab</span>",
            volta_short: "<span> Vale Ab</span><span>ğŸ –</span><span class='ultimo'>PoceirÃ£o</span>",
            circular_short: ""
        },
        4306: {
            ida: "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
            volta: "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
            circular: "",
            ida_short: "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
            volta_short: "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
            circular_short: ""
        },
        4313: {
            ida: "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Penalva</span>",
            volta: "<span> Penalva</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
            circular: "",
            ida_short: "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Penalva</span>",
            volta_short: "<span> Penalva</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
            circular_short: ""
        },
        4308: {
            ida: "<span>Palmela (Terminal) </span><span>ğŸ –</span><span class='ultimo'> Pinhal Novo (EstaÃ§Ã£o)</span>",
            volta: "<span> Pinhal Novo (EstaÃ§Ã£o)</span><span>ğŸ –</span><span class='ultimo'>Palmela (Terminal) </span>",
            circular: "",
            ida_short: "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> Pinhal </span>",
            volta_short: "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
            circular_short: ""
        },
        4320: {
            ida: "",
            volta: "",
            circular: "Pinhal Novo | Circular",
            ida_short: "",
            volta_short: "",
            circular_short: "Pinhal Novo | Ci"
        },
        4321: {
            ida: "<span>Pinhal Novo </span><span>ğŸ –</span><span class='ultimo'> Qta do Anjo</span>",
            volta: "<span> Qta do Anjo</span><span>ğŸ –</span><span class='ultimo'>Pinhal Novo </span>",
            circular: "",
            ida_short: "<span>Pinhal N</span><span>ğŸ –</span><span class='ultimo'> Qta do </span>",
            volta_short: "<span> Qta do </span><span>ğŸ –</span><span class='ultimo'>Pinhal N</span>",
            circular_short: ""
        },
        4432: {
            ida: "<span>SetÃºbal (ITS) </span><span>ğŸ –</span><span class='ultimo'> Vale de Choupo</span>",
            volta: "<span> Vale de Choupo</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (ITS) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Vale de</span>",
            volta_short: "<span> Vale de</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4412: {
            ida: "<span>Morgada </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Mercado)</span>",
            volta: "<span> SetÃºbal (Mercado)</span><span>ğŸ –</span><span class='ultimo'>Morgada </span>",
            circular: "",
            ida_short: "<span>Morgada </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Morgada </span>",
            circular_short: ""
        },
        4413: {
            ida: "",
            volta: "<span> SetÃºbal (Mercado), via Bela Vista</span><span>ğŸ –</span><span class='ultimo'>Morgada </span>",
            circular: "",
            ida_short: "",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Morgada </span>",
            circular_short: ""
        },
        4406: {
            ida: "<span>Manteigadas </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Mercado)</span>",
            volta: "<span> SetÃºbal (Mercado)</span><span>ğŸ –</span><span class='ultimo'>Manteigadas </span>",
            circular: "",
            ida_short: "<span>Manteiga</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Manteiga</span>",
            circular_short: ""
        },
        4408: {
            ida: "",
            volta: "<span> SetÃºbal (Mercado), via Bela Vista</span><span>ğŸ –</span><span class='ultimo'>Manteigadas </span>",
            circular: "",
            ida_short: "",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Manteiga</span>",
            circular_short: ""
        },
        4436: {
            ida: "<span>SetÃºbal (Mercado) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Av. Soeiro Pereira Gomes)</span>",
            volta: "<span> SetÃºbal (Av. Soeiro Pereira Gomes)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Mercado) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4420: {
            ida: "<span>SetÃºbal (Alegro) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Alegro) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4416: {
            ida: "<span>PoÃ§o Mouro </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>PoÃ§o Mouro </span>",
            circular: "",
            ida_short: "<span>PoÃ§o Mou</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>PoÃ§o Mou</span>",
            circular_short: ""
        },
        4431: {
            ida: "<span>SetÃºbal (ITS) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Quinta Varzinha)</span>",
            volta: "<span> SetÃºbal (Quinta Varzinha)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (ITS) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4434: {
            ida: "<span>SetÃºbal (Mercado 2 de Abril) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (R. Timor)</span>",
            volta: "<span> SetÃºbal (R. Timor)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Mercado 2 de Abril) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4426: {
            ida: "<span>SetÃºbal (Bairro Viso) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (CHEsetÃºbal)</span>",
            volta: "<span> SetÃºbal (CHEsetÃºbal)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Bairro Viso) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4425: {
            ida: "<span>SetÃºbal (Escola Viso) </span><span>ğŸ –</span><span class='ultimo'> Mitrena</span>",
            volta: "<span> Mitrena</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Escola Viso) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Mitrena</span>",
            volta_short: "<span> Mitrena</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4424: {
            ida: "<span>SetÃºbal (Bairro Viso) </span><span>ğŸ –</span><span class='ultimo'> Manteigadas</span>",
            volta: "<span> Manteigadas</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Bairro Viso) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Manteig</span>",
            volta_short: "<span> Manteig</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4422: {
            ida: "<span>SetÃºbal (Bairro Camolas) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Casal Figueiras), via Bairro do Viso</span>",
            volta: "<span> SetÃºbal (Casal Figueiras), via Bairro do Viso</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Bairro Camolas) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4428: {
            ida: "<span>SetÃºbal (Casal Figueiras) </span><span>ğŸ –</span><span class='ultimo'> Vale Ana Gomes</span>",
            volta: "<span> Vale Ana Gomes</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Casal Figueiras) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Vale An</span>",
            volta_short: "<span> Vale An</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4421: {
            ida: "<span>SetÃºbal (Bairro Camolas) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Casal Figueiras)</span>",
            volta: "<span> SetÃºbal (Casal Figueiras)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Bairro Camolas) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4440: {
            ida: "<span>SetÃºbal (Monte Belo Norte) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Saboaria), via Alegro</span>",
            volta: "<span> SetÃºbal (Saboaria), via Alegro</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Monte Belo Norte) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4438: {
            ida: "<span>SetÃºbal (Monte Belo Norte) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Saboaria)</span>",
            volta: "<span> SetÃºbal (Saboaria)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Monte Belo Norte) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4419: {
            ida: "<span>Brejos Canes </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Saboaria)</span>",
            volta: "<span> SetÃºbal (Saboaria)</span><span>ğŸ –</span><span class='ultimo'>Brejos Canes </span>",
            circular: "",
            ida_short: "<span>Brejos C</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Brejos C</span>",
            circular_short: ""
        },
        4441: {
            ida: "<span>SetÃºbal (Saboaria) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Vale Cobro)</span>",
            volta: "<span> SetÃºbal (Vale Cobro)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Saboaria) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4433: {
            ida: "<span>Alto Guerra </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Casal Figueiras)</span>",
            volta: "<span> SetÃºbal (Casal Figueiras)</span><span>ğŸ –</span><span class='ultimo'>Alto Guerra </span>",
            circular: "",
            ida_short: "<span>Alto Gue</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Alto Gue</span>",
            circular_short: ""
        },
        4407: {
            ida: "<span>Manteigadas </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Mercado), via Bairro da Carmona</span>",
            volta: "",
            circular: "",
            ida_short: "<span>Manteiga</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "",
            circular_short: ""
        },
        4472: {
            ida: "<span>Praia do Creiro </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Praia do Creiro </span>",
            circular: "",
            ida_short: "<span>Praia do</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Praia do</span>",
            circular_short: ""
        },
        4474: {
            ida: "<span>Figueirinha </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Alegro)</span>",
            volta: "<span> SetÃºbal (Alegro)</span><span>ğŸ –</span><span class='ultimo'>Figueirinha </span>",
            circular: "",
            ida_short: "<span>Figueiri</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Figueiri</span>",
            circular_short: ""
        },
        4471: {
            ida: "",
            volta: "",
            circular: "Praia Albarquel | Circular",
            ida_short: "",
            volta_short: "",
            circular_short: "Praia Albarquel "
        },
        4470: {
            ida: "<span>Brejos AzeitÃ£o </span><span>ğŸ –</span><span class='ultimo'> Praia do Creiro</span>",
            volta: "<span> Praia do Creiro</span><span>ğŸ –</span><span class='ultimo'>Brejos AzeitÃ£o </span>",
            circular: "",
            ida_short: "<span>Brejos A</span><span>ğŸ –</span><span class='ultimo'> Praia d</span>",
            volta_short: "<span> Praia d</span><span>ğŸ –</span><span class='ultimo'>Brejos A</span>",
            circular_short: ""
        },
        4414: {
            ida: "<span>OutÃ£o (Hospital) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>OutÃ£o (Hospital) </span>",
            circular: "",
            ida_short: "<span>OutÃ£o (H</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>OutÃ£o (H</span>",
            circular_short: ""
        },
        4415: {
            ida: "<span>OutÃ£o (Hospital) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS), via vale da Rasca</span>",
            volta: "<span> SetÃºbal (ITS), via vale da Rasca</span><span>ğŸ –</span><span class='ultimo'>OutÃ£o (Hospital) </span>",
            circular: "",
            ida_short: "<span>OutÃ£o (H</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>OutÃ£o (H</span>",
            circular_short: ""
        },
        4452: {
            ida: "<span>Mitrena (Portucel) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Mitrena (Portucel) </span>",
            circular: "",
            ida_short: "<span>Mitrena </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Mitrena </span>",
            circular_short: ""
        },
        4453: {
            ida: "<span>Mitrena (Portucel) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS), via Estrada GraÃ§a</span>",
            volta: "<span> SetÃºbal (ITS), via Estrada GraÃ§a</span><span>ğŸ –</span><span class='ultimo'>Mitrena (Portucel) </span>",
            circular: "",
            ida_short: "<span>Mitrena </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Mitrena </span>",
            circular_short: ""
        },
        4442: {
            ida: "<span>Praias do Sado (EstaÃ§Ã£o) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Bela Vista)</span>",
            volta: "<span> SetÃºbal (Bela Vista)</span><span>ğŸ –</span><span class='ultimo'>Praias do Sado (EstaÃ§Ã£o) </span>",
            circular: "",
            ida_short: "<span>Praias d</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Praias d</span>",
            circular_short: ""
        },
        4451: {
            ida: "<span>Mitrena (Lisnave) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Mitrena (Lisnave) </span>",
            circular: "",
            ida_short: "<span>Mitrena </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Mitrena </span>",
            circular_short: ""
        },
        4402: {
            ida: "<span>Estefanilha </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Estefanilha </span>",
            circular: "",
            ida_short: "<span>Estefani</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Estefani</span>",
            circular_short: ""
        },
        4437: {
            ida: "<span>FaralhÃ£o </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>FaralhÃ£o </span>",
            circular: "",
            ida_short: "<span>FaralhÃ£o</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>FaralhÃ£o</span>",
            circular_short: ""
        },
        4411: {
            ida: "<span>Morgada </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Morgada </span>",
            circular: "",
            ida_short: "<span>Morgada </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Morgada </span>",
            circular_short: ""
        },
        4439: {
            ida: "<span>Praias do Sado </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Praias do Sado </span>",
            circular: "",
            ida_short: "<span>Praias d</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Praias d</span>",
            circular_short: ""
        },
        4460: {
            ida: "AzeitÃ£o | Circular",
            volta: "AzeitÃ£o | Circular",
            circular: "",
            ida_short: "",
            volta_short: "AzeitÃ£o | Circul",
            circular_short: ""
        },
        4435: {
            ida: "<span>Biscainho </span><span>ğŸ –</span><span class='ultimo'> FaralhÃ£o</span>",
            volta: "<span> FaralhÃ£o</span><span>ğŸ –</span><span class='ultimo'>Biscainho </span>",
            circular: "",
            ida_short: "<span>Biscainh</span><span>ğŸ –</span><span class='ultimo'> FaralhÃ£</span>",
            volta_short: "<span> FaralhÃ£</span><span>ğŸ –</span><span class='ultimo'>Biscainh</span>",
            circular_short: ""
        },
        4429: {
            ida: "<span>SetÃºbal (Centro SaÃºde) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Mercado)</span>",
            volta: "<span> SetÃºbal (Mercado)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Centro SaÃºde) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4410: {
            ida: "<span>Manteigadas (Esc. Profissional) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Alegro)</span>",
            volta: "<span> SetÃºbal (Alegro)</span><span>ğŸ –</span><span class='ultimo'>Manteigadas (Esc. Profissional) </span>",
            circular: "",
            ida_short: "<span>Manteiga</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Manteiga</span>",
            circular_short: ""
        },
        4404: {
            ida: "",
            volta: "",
            circular: "Interfaces SetÃºbal | Circular",
            ida_short: "",
            volta_short: "",
            circular_short: "Interfaces SetÃºb"
        },
        4409: {
            ida: "<span>Manteigadas </span><span>ğŸ –</span><span class='ultimo'> Viso</span>",
            volta: "<span> Viso</span><span>ğŸ –</span><span class='ultimo'>Manteigadas </span>",
            circular: "",
            ida_short: "<span>Manteiga</span><span>ğŸ –</span><span class='ultimo'> Viso</span>",
            volta_short: "<span> Viso</span><span>ğŸ –</span><span class='ultimo'>Manteiga</span>",
            circular_short: ""
        },
        4427: {
            ida: "<span>SetÃºbal (Bela Vista) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Mercado)</span>",
            volta: "<span> SetÃºbal (Mercado)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Bela Vista) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4430: {
            ida: "<span>SetÃºbal (Hospital) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (MontalvÃ£o)</span>",
            volta: "<span> SetÃºbal (MontalvÃ£o)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Hospital) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4401: {
            ida: "<span>Cachofarra </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Hospital)</span>",
            volta: "<span> SetÃºbal (Hospital)</span><span>ğŸ –</span><span class='ultimo'>Cachofarra </span>",
            circular: "",
            ida_short: "<span>Cachofar</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Cachofar</span>",
            circular_short: ""
        },
        4443: {
            ida: "<span>SetÃºbal (PolitÃ©cnico) </span><span>ğŸ –</span><span class='ultimo'> Praias do Sado</span>",
            volta: "<span> Praias do Sado</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (PolitÃ©cnico) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Praias </span>",
            volta_short: "<span> Praias </span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4475: {
            ida: "<span>Portinho da ArrÃ¡bida </span><span>ğŸ –</span><span class='ultimo'> Viso</span>",
            volta: "<span> Viso</span><span>ğŸ –</span><span class='ultimo'>Portinho da ArrÃ¡bida </span>",
            circular: "",
            ida_short: "<span>Portinho</span><span>ğŸ –</span><span class='ultimo'> Viso</span>",
            volta_short: "<span> Viso</span><span>ğŸ –</span><span class='ultimo'>Portinho</span>",
            circular_short: ""
        },
        4417: {
            ida: "<span>PoÃ§o Mouro </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS), via Manteigadas</span>",
            volta: "<span> SetÃºbal (ITS), via Manteigadas</span><span>ğŸ –</span><span class='ultimo'>PoÃ§o Mouro </span>",
            circular: "",
            ida_short: "<span>PoÃ§o Mou</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>PoÃ§o Mou</span>",
            circular_short: ""
        },
        4418: {
            ida: "<span>SetÃºbal (Alegro) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Av. 5 Outubro)</span>",
            volta: "<span> SetÃºbal (Av. 5 Outubro)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (Alegro) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4403: {
            ida: "<span>Fonte da Talha </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Av. LuÃ­sa Todi)</span>",
            volta: "<span> SetÃºbal (Av. LuÃ­sa Todi)</span><span>ğŸ –</span><span class='ultimo'>Fonte da Talha </span>",
            circular: "",
            ida_short: "<span>Fonte da</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Fonte da</span>",
            circular_short: ""
        },
        4423: {
            ida: "<span>Amoreiras </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Av. LuÃ­sa Todi)</span>",
            volta: "<span> SetÃºbal (Av. LuÃ­sa Todi)</span><span>ğŸ –</span><span class='ultimo'>Amoreiras </span>",
            circular: "",
            ida_short: "<span>Amoreira</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Amoreira</span>",
            circular_short: ""
        },
        4405: {
            ida: "Livramento-Montebelo | Circular",
            volta: "Livramento-Montebelo | Circular",
            circular: "",
            ida_short: "",
            volta_short: "Livramento-Monte",
            circular_short: ""
        },
        4476: {
            ida: "",
            volta: "",
            circular: "Praias ArrÃ¡bida | Circular",
            ida_short: "",
            volta_short: "",
            circular_short: "Praias ArrÃ¡bida "
        },
        4561: {
            ida: "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Vila Nogueira de AzeitÃ£o, via Quinta do PicÃ£o</span>",
            volta: "<span> Vila Nogueira de AzeitÃ£o, via Quinta do PicÃ£o</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
            circular: "",
            ida_short: "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Vila No</span>",
            volta_short: "<span> Vila No</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
            circular_short: ""
        },
        4560: {
            ida: "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Vila Nogueira de AzeitÃ£o</span>",
            volta: "<span> Vila Nogueira de AzeitÃ£o</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
            circular: "",
            ida_short: "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> Vila No</span>",
            volta_short: "<span> Vila No</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
            circular_short: ""
        },
        4532: {
            ida: "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Quatro Marcos</span>",
            volta: "<span> Quatro Marcos</span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
            circular: "",
            ida_short: "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Quatro </span>",
            volta_short: "<span> Quatro </span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
            circular_short: ""
        },
        4531: {
            ida: "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
            volta: "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
            circular: "",
            ida_short: "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
            volta_short: "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
            circular_short: ""
        },
        4503: {
            ida: "<span>Atalaia </span><span>ğŸ –</span><span class='ultimo'> Jardia</span>",
            volta: "<span> Jardia</span><span>ğŸ –</span><span class='ultimo'>Atalaia </span>",
            circular: "",
            ida_short: "<span>Atalaia </span><span>ğŸ –</span><span class='ultimo'> Jardia</span>",
            volta_short: "<span> Jardia</span><span>ğŸ –</span><span class='ultimo'>Atalaia </span>",
            circular_short: ""
        },
        4504: {
            ida: "<span>Montijo (Terminal Fluvial) </span><span>ğŸ –</span><span class='ultimo'> Passil</span>",
            volta: "<span> Passil</span><span>ğŸ –</span><span class='ultimo'>Montijo (Terminal Fluvial) </span>",
            circular: "",
            ida_short: "<span>Montijo </span><span>ğŸ –</span><span class='ultimo'> Passil</span>",
            volta_short: "<span> Passil</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
            circular_short: ""
        },
        4551: {
            ida: "<span>Palmela (USF) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Av. LuÃ­sa Todi)</span>",
            volta: "<span> SetÃºbal (Av. LuÃ­sa Todi)</span><span>ğŸ –</span><span class='ultimo'>Palmela (USF) </span>",
            circular: "",
            ida_short: "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
            circular_short: ""
        },
        4501: {
            ida: "<span>Alcochete </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal Fluvial)</span>",
            volta: "<span> Montijo (Terminal Fluvial)</span><span>ğŸ –</span><span class='ultimo'>Alcochete </span>",
            circular: "",
            ida_short: "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
            volta_short: "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
            circular_short: ""
        },
        4512: {
            ida: "<span>Alcochete (Freeport) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS), via Alto Estanqueiro</span>",
            volta: "<span> SetÃºbal (ITS), via Alto Estanqueiro</span><span>ğŸ –</span><span class='ultimo'>Alcochete (Freeport) </span>",
            circular: "",
            ida_short: "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
            circular_short: ""
        },
        4513: {
            ida: "<span>Alcochete (Freeport) </span><span>ğŸ –</span><span class='ultimo'> Pinhal Novo</span>",
            volta: "<span> Pinhal Novo</span><span>ğŸ –</span><span class='ultimo'>Alcochete (Freeport) </span>",
            circular: "",
            ida_short: "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> Pinhal </span>",
            volta_short: "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
            circular_short: ""
        },
        4510: {
            ida: "<span>Alcochete (Freeport) </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal RodoviÃ¡rio)</span>",
            volta: "<span> Montijo (Terminal RodoviÃ¡rio)</span><span>ğŸ –</span><span class='ultimo'>Alcochete (Freeport) </span>",
            circular: "",
            ida_short: "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
            volta_short: "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
            circular_short: ""
        },
        4523: {
            ida: "",
            volta: "<span> Pinhal Novo</span><span>ğŸ –</span><span class='ultimo'>Montijo (Terminal RodoviÃ¡rio) </span>",
            circular: "",
            ida_short: "",
            volta_short: "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
            circular_short: ""
        },
        4517: {
            ida: "",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Montijo (Terminal RodoviÃ¡rio) </span>",
            circular: "",
            ida_short: "",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
            circular_short: ""
        },
        4514: {
            ida: "<span>Canha </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal RodoviÃ¡rio), via PegÃµes</span>",
            volta: "<span> Montijo (Terminal RodoviÃ¡rio), via PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Canha </span>",
            circular: "",
            ida_short: "<span>Canha </span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
            volta_short: "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Canha </span>",
            circular_short: ""
        },
        4515: {
            ida: "<span>Montijo (Terminal RodoviÃ¡rio) </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
            volta: "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Montijo (Terminal RodoviÃ¡rio) </span>",
            circular: "",
            ida_short: "<span>Montijo </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
            volta_short: "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
            circular_short: ""
        },
        4502: {
            ida: "<span>Alcochete </span><span>ğŸ –</span><span class='ultimo'> Passil</span>",
            volta: "<span> Passil</span><span>ğŸ –</span><span class='ultimo'>Alcochete </span>",
            circular: "",
            ida_short: "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> Passil</span>",
            volta_short: "<span> Passil</span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
            circular_short: ""
        },
        4516: {
            ida: "<span>Montijo (Terminal RodoviÃ¡rio) </span><span>ğŸ –</span><span class='ultimo'> Rio Frio</span>",
            volta: "<span> Rio Frio</span><span>ğŸ –</span><span class='ultimo'>Montijo (Terminal RodoviÃ¡rio) </span>",
            circular: "",
            ida_short: "<span>Montijo </span><span>ğŸ –</span><span class='ultimo'> Rio Fri</span>",
            volta_short: "<span> Rio Fri</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
            circular_short: ""
        },
        4520: {
            ida: "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
            volta: "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
            circular: "",
            ida_short: "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
            volta_short: "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
            circular_short: ""
        },
        4521: {
            ida: "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> Pinhal Novo</span>",
            volta: "<span> Pinhal Novo</span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
            circular: "",
            ida_short: "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> Pinhal </span>",
            volta_short: "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
            circular_short: ""
        },
        4522: {
            ida: "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> PoceirÃ£o</span>",
            volta: "<span> PoceirÃ£o</span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
            circular: "",
            ida_short: "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> PoceirÃ£</span>",
            volta_short: "<span> PoceirÃ£</span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
            circular_short: ""
        },
        4524: {
            ida: "<span>Palmela (Terminal) </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
            volta: "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Palmela (Terminal) </span>",
            circular: "",
            ida_short: "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
            volta_short: "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
            circular_short: ""
        },
        4530: {
            ida: "<span>Bairro Vila Morena </span><span>ğŸ –</span><span class='ultimo'> Pinhal Novo</span>",
            volta: "<span> Pinhal Novo</span><span>ğŸ –</span><span class='ultimo'>Bairro Vila Morena </span>",
            circular: "",
            ida_short: "<span>Bairro V</span><span>ğŸ –</span><span class='ultimo'> Pinhal </span>",
            volta_short: "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Bairro V</span>",
            circular_short: ""
        },
        4541: {
            ida: "<span>Algeruz </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Av. LuÃ­sa Todi)</span>",
            volta: "<span> SetÃºbal (Av. LuÃ­sa Todi)</span><span>ğŸ –</span><span class='ultimo'>Algeruz </span>",
            circular: "",
            ida_short: "<span>Algeruz </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Algeruz </span>",
            circular_short: ""
        },
        4545: {
            ida: "<span>Biscainho </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Bela Vista)</span>",
            volta: "<span> SetÃºbal (Bela Vista)</span><span>ğŸ –</span><span class='ultimo'>Biscainho </span>",
            circular: "",
            ida_short: "<span>Biscainh</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Biscainh</span>",
            circular_short: ""
        },
        4548: {
            ida: "<span>LagameÃ§as </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>LagameÃ§as </span>",
            circular: "",
            ida_short: "<span>LagameÃ§a</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>LagameÃ§a</span>",
            circular_short: ""
        },
        4544: {
            ida: "<span>Bairro MargaÃ§a </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Bairro MargaÃ§a </span>",
            circular: "",
            ida_short: "<span>Bairro M</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Bairro M</span>",
            circular_short: ""
        },
        4540: {
            ida: "<span>Ã�guas de Moura </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Ã�guas de Moura </span>",
            circular: "",
            ida_short: "<span>Ã�guas de</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Ã�guas de</span>",
            circular_short: ""
        },
        4547: {
            ida: "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
            circular: "",
            ida_short: "<span>Cabanas </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Cabanas </span>",
            circular_short: ""
        },
        4562: {
            ida: "<span>SetÃºbal (ITS) </span><span>ğŸ –</span><span class='ultimo'> Vila Nogueira de AzeitÃ£o, via Palmela (EstaÃ§Ã£o)</span>",
            volta: "<span> Vila Nogueira de AzeitÃ£o, via Palmela (EstaÃ§Ã£o)</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (ITS) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Vila No</span>",
            volta_short: "<span> Vila No</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        },
        4549: {
            ida: "<span>Palmela (Terminal) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Palmela (Terminal) </span>",
            circular: "",
            ida_short: "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
            circular_short: ""
        },
        4550: {
            ida: "<span>Palmela (Terminal) </span><span>ğŸ –</span><span class='ultimo'> Vila Nogueira de AzeitÃ£o</span>",
            volta: "<span> Vila Nogueira de AzeitÃ£o</span><span>ğŸ –</span><span class='ultimo'>Palmela (Terminal) </span>",
            circular: "",
            ida_short: "<span>Palmela </span><span>ğŸ –</span><span class='ultimo'> Vila No</span>",
            volta_short: "<span> Vila No</span><span>ğŸ –</span><span class='ultimo'>Palmela </span>",
            circular_short: ""
        },
        4546: {
            ida: "<span>Biscainho </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Biscainho </span>",
            circular: "",
            ida_short: "<span>Biscainh</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Biscainh</span>",
            circular_short: ""
        },
        4542: {
            ida: "<span>Algeruz </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Algeruz </span>",
            circular: "",
            ida_short: "<span>Algeruz </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Algeruz </span>",
            circular_short: ""
        },
        4543: {
            ida: "<span>Algeruz </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS), via PoÃ§oilos</span>",
            volta: "<span> SetÃºbal (ITS), via PoÃ§oilos</span><span>ğŸ –</span><span class='ultimo'>Algeruz </span>",
            circular: "",
            ida_short: "<span>Algeruz </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Algeruz </span>",
            circular_short: ""
        },
        4511: {
            ida: "<span>Alcochete (Freeport) </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal RodoviÃ¡rio), via Samouco</span>",
            volta: "<span> Montijo (Terminal RodoviÃ¡rio), via Samouco</span><span>ğŸ –</span><span class='ultimo'>Alcochete (Freeport) </span>",
            circular: "",
            ida_short: "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
            volta_short: "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
            circular_short: ""
        },
        4642: {
            ida: "<span>Sesimbra (Terminal) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (Hospital)</span>",
            volta: "<span> SetÃºbal (Hospital)</span><span>ğŸ –</span><span class='ultimo'>Sesimbra (Terminal) </span>",
            circular: "",
            ida_short: "<span>Sesimbra</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Sesimbra</span>",
            circular_short: ""
        },
        4621: {
            ida: "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Seixal (Terminal Fluvial)</span>",
            volta: "<span> Seixal (Terminal Fluvial)</span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
            circular: "",
            ida_short: "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Seixal </span>",
            volta_short: "<span> Seixal </span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
            circular_short: ""
        },
        4640: {
            ida: "<span>Casais da Serra </span><span>ğŸ –</span><span class='ultimo'> Vila Nogueira de AzeitÃ£o</span>",
            volta: "<span> Vila Nogueira de AzeitÃ£o</span><span>ğŸ –</span><span class='ultimo'>Casais da Serra </span>",
            circular: "",
            ida_short: "<span>Casais d</span><span>ğŸ –</span><span class='ultimo'> Vila No</span>",
            volta_short: "<span> Vila No</span><span>ğŸ –</span><span class='ultimo'>Casais d</span>",
            circular_short: ""
        },
        4620: {
            ida: "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Paio Pires</span>",
            volta: "<span> Paio Pires</span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
            circular: "",
            ida_short: "<span>Moita </span><span>ğŸ –</span><span class='ultimo'> Paio Pi</span>",
            volta_short: "<span> Paio Pi</span><span>ğŸ –</span><span class='ultimo'>Moita </span>",
            circular_short: ""
        },
        4643: {
            ida: "<span>Montijo (Av. Inf. D. Henrique) </span><span>ğŸ –</span><span class='ultimo'> Sesimbra (Terminal)</span>",
            volta: "<span> Sesimbra (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Montijo (Av. Inf. D. Henrique) </span>",
            circular: "",
            ida_short: "<span>Montijo </span><span>ğŸ –</span><span class='ultimo'> Sesimbr</span>",
            volta_short: "<span> Sesimbr</span><span>ğŸ –</span><span class='ultimo'>Montijo </span>",
            circular_short: ""
        },
        4631: {
            ida: "<span>Fogueteiro (EstaÃ§Ã£o) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Fogueteiro (EstaÃ§Ã£o) </span>",
            circular: "",
            ida_short: "<span>Foguetei</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Foguetei</span>",
            circular_short: ""
        },
        4630: {
            ida: "<span>Corroios (EstaÃ§Ã£o) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Corroios (EstaÃ§Ã£o) </span>",
            circular: "",
            ida_short: "<span>Corroios</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Corroios</span>",
            circular_short: ""
        },
        4641: {
            ida: "<span>Quinta do Conde </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Quinta do Conde </span>",
            circular: "",
            ida_short: "<span>Quinta d</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Quinta d</span>",
            circular_short: ""
        },
        4610: {
            ida: "<span>Bairro dos Marinheiros </span><span>ğŸ –</span><span class='ultimo'> Barreiro (Terminal)</span>",
            volta: "<span> Barreiro (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Bairro dos Marinheiros </span>",
            circular: "",
            ida_short: "<span>Bairro d</span><span>ğŸ –</span><span class='ultimo'> Barreir</span>",
            volta_short: "<span> Barreir</span><span>ğŸ –</span><span class='ultimo'>Bairro d</span>",
            circular_short: ""
        },
        4602: {
            ida: "<span>Alhos Vedros (EstaÃ§Ã£o) </span><span>ğŸ –</span><span class='ultimo'> Barreiro (Terminal)</span>",
            volta: "<span> Barreiro (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Alhos Vedros (EstaÃ§Ã£o) </span>",
            circular: "",
            ida_short: "<span>Alhos Ve</span><span>ğŸ –</span><span class='ultimo'> Barreir</span>",
            volta_short: "<span> Barreir</span><span>ğŸ –</span><span class='ultimo'>Alhos Ve</span>",
            circular_short: ""
        },
        4612: {
            ida: "<span>Bairro dos Marinheiros </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
            volta: "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Bairro dos Marinheiros </span>",
            circular: "",
            ida_short: "<span>Bairro d</span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
            volta_short: "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Bairro d</span>",
            circular_short: ""
        },
        4605: {
            ida: "<span>Lavradio </span><span>ğŸ –</span><span class='ultimo'> Pinhal do Forno</span>",
            volta: "<span> Pinhal do Forno</span><span>ğŸ –</span><span class='ultimo'>Lavradio </span>",
            circular: "",
            ida_short: "<span>Lavradio</span><span>ğŸ –</span><span class='ultimo'> Pinhal </span>",
            volta_short: "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Lavradio</span>",
            circular_short: ""
        },
        4600: {
            ida: "<span>Alcochete (Freeport) </span><span>ğŸ –</span><span class='ultimo'> Barreiro (Terminal)</span>",
            volta: "<span> Barreiro (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Alcochete (Freeport) </span>",
            circular: "",
            ida_short: "<span>Alcochet</span><span>ğŸ –</span><span class='ultimo'> Barreir</span>",
            volta_short: "<span> Barreir</span><span>ğŸ –</span><span class='ultimo'>Alcochet</span>",
            circular_short: ""
        },
        4604: {
            ida: "<span>Barreiro (Terminal) </span><span>ğŸ –</span><span class='ultimo'> Moita (Escola Fragata do Tejo)</span>",
            volta: "<span> Moita (Escola Fragata do Tejo)</span><span>ğŸ –</span><span class='ultimo'>Barreiro (Terminal) </span>",
            circular: "",
            ida_short: "<span>Barreiro</span><span>ğŸ –</span><span class='ultimo'> Moita (</span>",
            volta_short: "<span> Moita (</span><span>ğŸ –</span><span class='ultimo'>Barreiro</span>",
            circular_short: ""
        },
        4611: {
            ida: "<span>Penalva </span><span>ğŸ –</span><span class='ultimo'> Moita (Esc. SecundÃ¡ria)</span>",
            volta: "<span> Moita (Esc. SecundÃ¡ria)</span><span>ğŸ –</span><span class='ultimo'>Penalva </span>",
            circular: "",
            ida_short: "<span>Penalva </span><span>ğŸ –</span><span class='ultimo'> Moita (</span>",
            volta_short: "<span> Moita (</span><span>ğŸ –</span><span class='ultimo'>Penalva </span>",
            circular_short: ""
        },
        4102: {
            ida: "<span>CabeÃ§o Verde </span><span>ğŸ –</span><span class='ultimo'> Sarilhos Pequenos</span>",
            volta: "<span> Sarilhos Pequenos</span><span>ğŸ –</span><span class='ultimo'>CabeÃ§o Verde </span>",
            circular: "",
            ida_short: "<span>CabeÃ§o V</span><span>ğŸ –</span><span class='ultimo'> Sarilho</span>",
            volta_short: "<span> Sarilho</span><span>ğŸ –</span><span class='ultimo'>CabeÃ§o V</span>",
            circular_short: ""
        },
        4603: {
            ida: "<span>Barreiro (Terminal) </span><span>ğŸ –</span><span class='ultimo'> ChÃ£o Duro</span>",
            volta: "<span> ChÃ£o Duro</span><span>ğŸ –</span><span class='ultimo'>Barreiro (Terminal) </span>",
            circular: "",
            ida_short: "<span>Barreiro</span><span>ğŸ –</span><span class='ultimo'> ChÃ£o Du</span>",
            volta_short: "<span> ChÃ£o Du</span><span>ğŸ –</span><span class='ultimo'>Barreiro</span>",
            circular_short: ""
        },
        4601: {
            ida: "<span>Barreiro (Terminal) </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal RodoviÃ¡rio)</span>",
            volta: "<span> Montijo (Terminal RodoviÃ¡rio)</span><span>ğŸ –</span><span class='ultimo'>Barreiro (Terminal) </span>",
            circular: "",
            ida_short: "<span>Barreiro</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
            volta_short: "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Barreiro</span>",
            circular_short: ""
        },
        4701: {
            ida: "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> Vale da Amoreira</span>",
            volta: "<span> Vale da Amoreira</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
            circular: "",
            ida_short: "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> Vale da</span>",
            volta_short: "<span> Vale da</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
            circular_short: ""
        },
        4703: {
            ida: "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal RodoviÃ¡rio), via Alcochete e Samouco</span>",
            volta: "<span> Montijo (Terminal RodoviÃ¡rio), via Alcochete e Samouco</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
            circular: "",
            ida_short: "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
            volta_short: "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
            circular_short: ""
        },
        4704: {
            ida: "<span>Atalaia </span><span>ğŸ –</span><span class='ultimo'> Lisboa (Oriente)</span>",
            volta: "<span> Lisboa (Oriente)</span><span>ğŸ –</span><span class='ultimo'>Atalaia </span>",
            circular: "",
            ida_short: "<span>Atalaia </span><span>ğŸ –</span><span class='ultimo'> Lisboa </span>",
            volta_short: "<span> Lisboa </span><span>ğŸ –</span><span class='ultimo'>Atalaia </span>",
            circular_short: ""
        },
        4702: {
            ida: "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> Valbom</span>",
            volta: "<span> Valbom</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
            circular: "",
            ida_short: "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> Valbom</span>",
            volta_short: "<span> Valbom</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
            circular_short: ""
        },
        4705: {
            ida: "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> Samouco</span>",
            volta: "<span> Samouco</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
            circular: "",
            ida_short: "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> Samouco</span>",
            volta_short: "<span> Samouco</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
            circular_short: ""
        },
        4707: {
            ida: "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> Montijo (Terminal RodoviÃ¡rio)</span>",
            volta: "<span> Montijo (Terminal RodoviÃ¡rio)</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
            circular: "",
            ida_short: "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> Montijo</span>",
            volta_short: "<span> Montijo</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
            circular_short: ""
        },
        4725: {
            ida: "<span>Lisboa (Sete Rios) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Sete Rios) </span>",
            circular: "",
            ida_short: "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
            circular_short: ""
        },
        4720: {
            ida: "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
            circular: "",
            ida_short: "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
            circular_short: ""
        },
        4715: {
            ida: "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS), via Pinhal Novo</span>",
            volta: "<span> SetÃºbal (ITS), via Pinhal Novo</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
            circular: "",
            ida_short: "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
            circular_short: ""
        },
        4710: {
            ida: "<span>Lisboa (Oriente) </span><span>ğŸ –</span><span class='ultimo'> Palmela (Terminal)</span>",
            volta: "<span> Palmela (Terminal)</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
            circular: "",
            ida_short: "<span>Lisboa (</span><span>ğŸ –</span><span class='ultimo'> Palmela</span>",
            volta_short: "<span> Palmela</span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
            circular_short: ""
        },
        4711: {
            ida: "",
            volta: "<span> Pinhal Novo</span><span>ğŸ –</span><span class='ultimo'>Lisboa (Oriente) </span>",
            circular: "",
            ida_short: "",
            volta_short: "<span> Pinhal </span><span>ğŸ –</span><span class='ultimo'>Lisboa (</span>",
            circular_short: ""
        },
        4706: {
            ida: "<span>SÃ£o Francisco </span><span>ğŸ –</span><span class='ultimo'> Lisboa (Oriente)</span>",
            volta: "<span> Lisboa (Oriente)</span><span>ğŸ –</span><span class='ultimo'>SÃ£o Francisco </span>",
            circular: "",
            ida_short: "<span>SÃ£o Fran</span><span>ğŸ –</span><span class='ultimo'> Lisboa </span>",
            volta_short: "<span> Lisboa </span><span>ğŸ –</span><span class='ultimo'>SÃ£o Fran</span>",
            circular_short: ""
        },
        4905: {
            ida: "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> Vendas Novas</span>",
            volta: "<span> Vendas Novas</span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
            circular: "",
            ida_short: "<span>Faias </span><span>ğŸ –</span><span class='ultimo'> Vendas </span>",
            volta_short: "<span> Vendas </span><span>ğŸ –</span><span class='ultimo'>Faias </span>",
            circular_short: ""
        },
        4901: {
            ida: "<span>Landeira </span><span>ğŸ –</span><span class='ultimo'> SetÃºbal (ITS)</span>",
            volta: "<span> SetÃºbal (ITS)</span><span>ğŸ –</span><span class='ultimo'>Landeira </span>",
            circular: "",
            ida_short: "<span>Landeira</span><span>ğŸ –</span><span class='ultimo'> SetÃºbal</span>",
            volta_short: "<span> SetÃºbal</span><span>ğŸ –</span><span class='ultimo'>Landeira</span>",
            circular_short: ""
        },
        4902: {
            ida: "<span>Landeira </span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
            volta: "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Landeira </span>",
            circular: "",
            ida_short: "<span>Landeira</span><span>ğŸ –</span><span class='ultimo'> PegÃµes</span>",
            volta_short: "<span> PegÃµes</span><span>ğŸ –</span><span class='ultimo'>Landeira</span>",
            circular_short: ""
        },
        4906: {
            ida: "<span>SetÃºbal (ITS) </span><span>ğŸ –</span><span class='ultimo'> Vendas Novas, via Landeira</span>",
            volta: "<span> Vendas Novas, via Landeira</span><span>ğŸ –</span><span class='ultimo'>SetÃºbal (ITS) </span>",
            circular: "",
            ida_short: "<span>SetÃºbal </span><span>ğŸ –</span><span class='ultimo'> Vendas </span>",
            volta_short: "<span> Vendas </span><span>ğŸ –</span><span class='ultimo'>SetÃºbal </span>",
            circular_short: ""
        }
    }, linhas_e_seus_horarios = {
        4001: {ida: [], volta: [], circular: ["4001_0_3.json"]},
        4002: {ida: [], volta: [], circular: ["4002_0_3.json"]},
        4101: {ida: ["4101_0_1.json"], volta: ["4101_0_2.json"], circular: []},
        4102: {ida: ["4102_0_1.json"], volta: ["4102_0_2.json"], circular: []},
        4103: {ida: ["4103_0_1.json"], volta: ["4103_0_2.json"], circular: []},
        4104: {ida: [], volta: [], circular: ["4104_0_3.json"]},
        4201: {ida: ["4201_0_1.json"], volta: ["4201_0_2.json"], circular: []},
        4202: {ida: ["4202_0_1.json"], volta: ["4202_0_2.json"], circular: []},
        4203: {ida: ["4203_0_1.json"], volta: ["4203_0_2.json", "4203_1_2.json"], circular: []},
        4204: {ida: ["4204_0_1.json"], volta: ["4204_0_2.json"], circular: []},
        4205: {ida: ["4205_0_1.json"], volta: ["4205_0_2.json", "4205_1_2.json"], circular: []},
        4206: {ida: ["4206_0_1.json"], volta: ["4206_0_2.json"], circular: []},
        4207: {ida: ["4207_0_1.json", "4207_1_1.json"], volta: ["4207_0_2.json", "4207_2_2.json"], circular: []},
        4208: {ida: ["4208_0_1.json"], volta: ["4208_0_2.json"], circular: []},
        4210: {ida: ["4210_0_1.json"], volta: ["4210_0_2.json"], circular: []},
        4211: {ida: [], volta: [], circular: ["4211_0_3.json"]},
        4212: {ida: ["4212_0_1.json"], volta: ["4212_0_2.json"], circular: []},
        4301: {ida: ["4301_0_1.json"], volta: ["4301_0_2.json"], circular: []},
        4302: {ida: ["4302_0_1.json"], volta: ["4302_0_2.json"], circular: []},
        4303: {ida: [], volta: [], circular: ["4303_0_3.json"]},
        4304: {ida: ["4304_0_1.json"], volta: ["4304_0_2.json"], circular: []},
        4305: {ida: ["4305_0_1.json"], volta: ["4305_0_2.json"], circular: []},
        4306: {ida: ["4306_0_1.json"], volta: ["4306_0_2.json"], circular: []},
        4307: {ida: ["4307_0_1.json"], volta: ["4307_0_2.json"], circular: []},
        4308: {ida: ["4308_0_1.json"], volta: ["4308_0_2.json"], circular: []},
        4310: {ida: ["4310_0_1.json"], volta: ["4310_0_2.json"], circular: []},
        4311: {ida: ["4311_0_1.json"], volta: ["4311_0_2.json"], circular: []},
        4312: {
            ida: ["4312_0_1.json", "4312_1_1.json"],
            volta: ["4312_0_2.json", "4312_1_2.json", "4312_2_2.json"],
            circular: []
        },
        4313: {ida: ["4313_0_1.json"], volta: ["4313_0_2.json"], circular: []},
        4320: {ida: [], volta: [], circular: ["4320_0_3.json"]},
        4321: {ida: ["4321_0_1.json"], volta: ["4321_0_2.json"], circular: []},
        4322: {ida: ["4322_0_1.json", "4322_1_1.json"], volta: ["4322_0_2.json", "4322_1_2.json"], circular: []},
        4401: {ida: ["4401_0_1.json"], volta: ["4401_0_2.json"], circular: []},
        4402: {ida: ["4402_0_1.json"], volta: ["4402_0_2.json"], circular: []},
        4403: {ida: ["4403_0_1.json"], volta: ["4403_0_2.json"], circular: []},
        4404: {ida: [], volta: [], circular: ["4404_0_3.json"]},
        4405: {ida: ["4405_0_1.json"], volta: ["4405_0_2.json"], circular: []},
        4406: {ida: ["4406_0_1.json", "4406_1_1.json"], volta: ["4406_0_2.json", "4406_1_2.json"], circular: []},
        4407: {ida: ["4407_0_1.json", "4407_1_1.json"], volta: [], circular: []},
        4408: {ida: [], volta: ["4408_0_2.json"], circular: []},
        4409: {ida: ["4409_0_1.json"], volta: ["4409_0_2.json"], circular: []},
        4410: {ida: ["4410_0_1.json"], volta: ["4410_0_2.json"], circular: []},
        4411: {ida: ["4411_0_1.json"], volta: ["4411_0_2.json"], circular: []},
        4412: {ida: ["4412_0_1.json"], volta: ["4412_0_2.json"], circular: []},
        4413: {ida: [], volta: ["4413_0_2.json"], circular: []},
        4414: {ida: ["4414_0_1.json"], volta: ["4414_0_2.json"], circular: []},
        4415: {ida: ["4415_0_1.json"], volta: ["4415_0_2.json"], circular: []},
        4416: {ida: ["4416_0_1.json"], volta: ["4416_0_2.json"], circular: []},
        4417: {ida: ["4417_0_1.json"], volta: ["4417_0_2.json"], circular: []},
        4418: {ida: ["4418_0_1.json"], volta: ["4418_0_2.json"], circular: []},
        4419: {ida: ["4419_0_1.json"], volta: ["4419_0_2.json"], circular: []},
        4420: {ida: ["4420_0_1.json"], volta: ["4420_0_2.json"], circular: []},
        4421: {ida: ["4421_0_1.json"], volta: ["4421_0_2.json"], circular: []},
        4422: {ida: ["4422_0_1.json", "4422_1_1.json"], volta: ["4422_0_2.json", "4422_2_2.json"], circular: []},
        4423: {ida: ["4423_0_1.json"], volta: ["4423_0_2.json"], circular: []},
        4424: {ida: ["4424_0_1.json"], volta: ["4424_0_2.json"], circular: []},
        4425: {ida: ["4425_0_1.json", "4425_1_1.json"], volta: ["4425_0_2.json", "4425_1_2.json"], circular: []},
        4426: {
            ida: ["4426_0_1.json", "4426_1_1.json", "4426_2_1.json"],
            volta: ["4426_0_2.json", "4426_1_2.json", "4426_3_2.json", "4426_4_2.json"],
            circular: []
        },
        4427: {ida: ["4427_0_1.json"], volta: ["4427_0_2.json"], circular: []},
        4428: {ida: ["4428_0_1.json"], volta: ["4428_0_2.json"], circular: []},
        4429: {ida: ["4429_0_1.json"], volta: ["4429_0_2.json"], circular: []},
        4430: {ida: ["4430_0_1.json"], volta: ["4430_0_2.json"], circular: []},
        4431: {ida: ["4431_0_1.json"], volta: ["4431_0_2.json"], circular: []},
        4432: {ida: ["4432_0_1.json"], volta: ["4432_0_2.json"], circular: []},
        4433: {ida: ["4433_0_1.json"], volta: ["4433_0_2.json"], circular: []},
        4434: {ida: ["4434_0_1.json", "4434_1_1.json"], volta: ["4434_0_2.json", "4434_1_2.json"], circular: []},
        4435: {ida: ["4435_0_1.json"], volta: ["4435_0_2.json"], circular: []},
        4436: {ida: ["4436_0_1.json"], volta: ["4436_0_2.json"], circular: []},
        4437: {ida: ["4437_0_1.json"], volta: ["4437_0_2.json"], circular: []},
        4438: {ida: ["4438_0_1.json", "4438_1_1.json"], volta: ["4438_0_2.json", "4438_1_2.json"], circular: []},
        4439: {ida: ["4439_0_1.json"], volta: ["4439_0_2.json"], circular: []},
        4440: {ida: ["4440_0_1.json"], volta: ["4440_0_2.json"], circular: []},
        4441: {ida: ["4441_0_1.json"], volta: ["4441_0_2.json"], circular: []},
        4442: {ida: ["4442_0_1.json", "4442_1_1.json"], volta: ["4442_0_2.json"], circular: []},
        4443: {ida: ["4443_0_1.json"], volta: ["4443_0_2.json"], circular: []},
        4451: {ida: ["4451_0_1.json"], volta: ["4451_0_2.json"], circular: []},
        4452: {ida: ["4452_0_1.json", "4452_1_1.json"], volta: ["4452_0_2.json", "4452_1_2.json"], circular: []},
        4453: {ida: ["4453_0_1.json"], volta: ["4453_0_2.json"], circular: []},
        4460: {ida: ["4460_0_1.json"], volta: ["4460_0_2.json"], circular: []},
        4470: {ida: ["4470_0_1.json"], volta: ["4470_0_2.json"], circular: []},
        4471: {ida: [], volta: [], circular: ["4471_0_3.json"]},
        4472: {ida: ["4472_0_1.json", "4472_1_1.json"], volta: ["4472_0_2.json", "4472_1_2.json"], circular: []},
        4474: {ida: ["4474_0_1.json"], volta: ["4474_0_2.json"], circular: []},
        4475: {ida: ["4475_0_1.json"], volta: ["4475_0_2.json", "4475_1_2.json"], circular: []},
        4476: {ida: [], volta: [], circular: ["4476_0_3.json"]},
        4501: {ida: ["4501_0_1.json", "4501_1_1.json"], volta: ["4501_0_2.json", "4501_1_2.json"], circular: []},
        4502: {ida: ["4502_0_1.json"], volta: ["4502_0_2.json"], circular: []},
        4503: {ida: ["4503_0_1.json"], volta: ["4503_0_2.json"], circular: []},
        4504: {ida: ["4504_0_1.json"], volta: ["4504_0_2.json"], circular: []},
        4510: {ida: ["4510_0_1.json"], volta: ["4510_0_2.json"], circular: []},
        4511: {ida: ["4511_0_1.json"], volta: ["4511_0_2.json"], circular: []},
        4512: {ida: ["4512_0_1.json", "4512_1_1.json"], volta: ["4512_0_2.json", "4512_1_2.json"], circular: []},
        4513: {ida: ["4513_0_1.json", "4513_1_1.json"], volta: ["4513_0_2.json"], circular: []},
        4514: {
            ida: ["4514_0_1.json", "4514_1_1.json", "4514_3_1.json"],
            volta: ["4514_0_2.json", "4514_2_2.json", "4514_3_2.json"],
            circular: []
        },
        4515: {ida: ["4515_0_1.json"], volta: ["4515_0_2.json", "4515_1_2.json"], circular: []},
        4516: {ida: ["4516_0_1.json", "4516_1_1.json"], volta: ["4516_0_2.json"], circular: []},
        4517: {ida: [], volta: ["4517_0_2.json"], circular: []},
        4520: {ida: ["4520_0_1.json"], volta: ["4520_0_2.json"], circular: []},
        4521: {ida: ["4521_0_1.json", "4521_1_1.json"], volta: ["4521_0_2.json"], circular: []},
        4522: {ida: ["4522_0_1.json"], volta: ["4522_0_2.json"], circular: []},
        4523: {ida: [], volta: ["4523_0_2.json"], circular: []},
        4524: {ida: ["4524_0_1.json", "4524_1_1.json"], volta: ["4524_0_2.json"], circular: []},
        4530: {ida: ["4530_0_1.json"], volta: ["4530_0_2.json"], circular: []},
        4531: {ida: ["4531_0_1.json"], volta: ["4531_0_2.json"], circular: []},
        4532: {ida: ["4532_0_1.json"], volta: ["4532_0_2.json"], circular: []},
        4540: {ida: ["4540_0_1.json", "4540_1_1.json"], volta: ["4540_0_2.json", "4540_1_2.json"], circular: []},
        4541: {ida: ["4541_0_1.json"], volta: ["4541_0_2.json"], circular: []},
        4542: {ida: ["4542_0_1.json"], volta: ["4542_0_2.json"], circular: []},
        4543: {ida: ["4543_0_1.json"], volta: ["4543_0_2.json"], circular: []},
        4544: {ida: ["4544_0_1.json", "4544_1_1.json"], volta: ["4544_0_2.json", "4544_2_2.json"], circular: []},
        4545: {ida: ["4545_0_1.json", "4545_1_1.json"], volta: ["4545_0_2.json", "4545_1_2.json"], circular: []},
        4546: {ida: ["4546_0_1.json"], volta: ["4546_0_2.json"], circular: []},
        4547: {ida: ["4547_0_1.json"], volta: ["4547_0_2.json"], circular: []},
        4548: {ida: ["4548_0_1.json"], volta: ["4548_0_2.json"], circular: []},
        4549: {ida: ["4549_0_1.json"], volta: ["4549_0_2.json"], circular: []},
        4550: {ida: ["4550_0_1.json"], volta: ["4550_0_2.json", "4550_1_2.json"], circular: []},
        4551: {ida: ["4551_0_1.json"], volta: ["4551_0_2.json"], circular: []},
        4560: {ida: ["4560_0_1.json", "4560_1_1.json"], volta: ["4560_0_2.json", "4560_1_2.json"], circular: []},
        4561: {ida: ["4561_0_1.json"], volta: ["4561_0_2.json"], circular: []},
        4562: {
            ida: ["4562_0_1.json", "4562_1_1.json", "4562_2_1.json", "4562_3_1.json", "4562_4_1.json", "4562_5_1.json"],
            volta: ["4562_0_2.json", "4562_2_2.json", "4562_3_2.json", "4562_5_2.json", "4562_7_2.json"],
            circular: []
        },
        4600: {ida: ["4600_0_1.json", "4600_1_1.json"], volta: ["4600_0_2.json", "4600_1_2.json"], circular: []},
        4601: {ida: ["4601_0_1.json", "4601_1_1.json"], volta: ["4601_0_2.json"], circular: []},
        4602: {ida: ["4602_0_1.json"], volta: ["4602_0_2.json"], circular: []},
        4603: {ida: ["4603_0_1.json"], volta: ["4603_0_2.json"], circular: []},
        4604: {ida: ["4604_0_1.json"], volta: ["4604_0_2.json"], circular: []},
        4605: {ida: ["4605_0_1.json"], volta: ["4605_0_2.json"], circular: []},
        4610: {ida: ["4610_0_1.json"], volta: ["4610_0_2.json"], circular: []},
        4611: {ida: ["4611_0_1.json"], volta: ["4611_0_2.json"], circular: []},
        4612: {ida: ["4612_0_1.json"], volta: ["4612_0_2.json"], circular: []},
        4620: {ida: ["4620_0_1.json"], volta: ["4620_0_2.json"], circular: []},
        4621: {ida: ["4621_0_1.json"], volta: ["4621_0_2.json"], circular: []},
        4630: {ida: ["4630_0_1.json"], volta: ["4630_0_2.json"], circular: []},
        4631: {ida: ["4631_0_1.json", "4631_1_1.json"], volta: ["4631_0_2.json", "4631_1_2.json"], circular: []},
        4640: {ida: ["4640_0_1.json"], volta: ["4640_0_2.json"], circular: []},
        4641: {ida: ["4641_0_1.json"], volta: ["4641_0_2.json"], circular: []},
        4642: {ida: ["4642_0_1.json", "4642_1_1.json"], volta: ["4642_0_2.json", "4642_1_2.json"], circular: []},
        4643: {ida: ["4643_0_1.json"], volta: ["4643_0_2.json"], circular: []},
        4701: {ida: ["4701_0_1.json"], volta: ["4701_0_2.json", "4701_1_2.json"], circular: []},
        4702: {ida: ["4702_0_1.json"], volta: ["4702_0_2.json"], circular: []},
        4703: {ida: ["4703_0_1.json"], volta: ["4703_0_2.json"], circular: []},
        4704: {ida: ["4704_0_1.json"], volta: ["4704_0_2.json"], circular: []},
        4705: {ida: ["4705_0_1.json"], volta: ["4705_0_2.json"], circular: []},
        4706: {ida: ["4706_0_1.json"], volta: ["4706_0_2.json"], circular: []},
        4707: {ida: ["4707_0_1.json"], volta: ["4707_0_2.json"], circular: []},
        4710: {ida: ["4710_0_1.json"], volta: ["4710_0_2.json"], circular: []},
        4711: {ida: [], volta: ["4711_0_2.json"], circular: []},
        4715: {ida: ["4715_0_1.json", "4715_1_1.json"], volta: ["4715_0_2.json", "4715_1_2.json"], circular: []},
        4720: {ida: ["4720_0_1.json", "4720_1_1.json"], volta: ["4720_0_2.json", "4720_1_2.json"], circular: []},
        4725: {ida: ["4725_0_1.json", "4725_1_1.json"], volta: ["4725_0_2.json", "4725_1_2.json"], circular: []},
        4901: {ida: ["4901_0_1.json", "4901_1_1.json"], volta: ["4901_0_2.json", "4901_1_2.json"], circular: []},
        4902: {ida: ["4902_0_1.json"], volta: ["4902_0_2.json"], circular: []},
        4905: {ida: ["4905_0_1.json"], volta: ["4905_0_2.json"], circular: []},
        4906: {ida: ["4906_0_1.json"], volta: ["4906_0_2.json", "4906_1_2.json"], circular: []}
    }, service_id = {
        1: "DiÃ¡rio",
        2: "Dias Ãºteis e domingos/feriados todo o ano",
        3: "Dias Ãºteis e sÃ¡bados todo o ano",
        4: "Dias Ãºteis todo o ano",
        5: "Domingos/feriados todo o ano",
        7: "SÃ¡bados (exceto feriados) todo o ano",
        8: "SÃ¡bados, domingos/feriados todo o ano",
        11: "Sextas-feiras e sÃ¡bados (exceto feriados) todo o ano",
        15: "Dias Ãºteis de perÃ­odo escolar",
        20: "Quartas, quintas e sextas-feiras (exceto feriados) de perÃ­odo escolar",
        21: "Quartas-feiras (exceto feriados) de perÃ­odo escolar",
        25: "Segundas e terÃ§as-feiras (exceto feriados) de perÃ­odo escolar",
        28: "Segundas, terÃ§as, quintas e sextas-feiras (exceto feriados) de perÃ­odo escolar",
        30: "Sextas-feiras (exceto feriados) de perÃ­odo escolar",
        36: "Dias Ãºteis exceto verÃ£o",
        38: "SÃ¡bados (exceto feriados) exceto verÃ£o",
        39: "SÃ¡bados, domingos/feriados exceto verÃ£o",
        40: "Todos os dias exceto verÃ£o",
        41: "Dias Ãºteis de fÃ©rias escolares e verÃ£o",
        50: "Dias Ãºteis de fÃ©rias escolares exceto verÃ£o",
        51: "Dias Ãºteis de verÃ£o",
        54: "SÃ¡bados, domingos/feriados de verÃ£o",
        56: "Todos os dias de verÃ£o",
        60: "Dias Ãºteis de fÃ©rias escolares e verÃ£o e sÃ¡bados (exceto feriados) todo o ano",
        62: "Dias Ãºteis de perÃ­odo escolar e sÃ¡bados (exceto feriados) de fÃ©rias escolares e verÃ£o",
        63: "Dias Ãºteis de perÃ­odo escolar e sÃ¡bados (exceto feriados) todo o ano",
        66: "Dias Ãºteis de perÃ­odo escolar e segundas-feiras (exceto feriados) de verÃ£o",
        69: "Segundo domingo de cada mÃªs",
        77: "Dias Ãºteis todo o ano e sÃ¡bados (exceto feriados) de fÃ©rias escolares e verÃ£o",
        87: "Domingos exceto primeiro de cada mÃªs todo o ano",
        97: "Primeiro domingo de cada mÃªs todo o ano",
        100: "SÃ¡bados (exceto feriados) todo o ano e dias Ãºteis de perÃ­odo escolar",
        102: "SÃ¡bados (exceto feriados) todo o ano e dias Ãºteis de verÃ£o",
        109: "Domingos/feriados todo o ano e dias Ãºteis de fÃ©rias escolares e verÃ£o",
        111: "SÃ¡bados, domingos/feriados todo o ano e dias Ãºteis de verÃ£o",
        112: "SÃ¡bados, domingos/feriados todo o ano e dias Ãºteis exceto verÃ£o",
        113: "Todos os dias de fÃ©rias escolares e verÃ£o domingos/feriados de perÃ­odo escolar",
        115: "Todos os dias de perÃ­odo escolar e sÃ¡bados, domingos/feriados de fÃ©rias escolares e verÃ£o",
        118: "Todos os dias entre 10 de junho e 15 de setembro",
        119: "SÃ¡bados, domingos/feriados entre 10 de junho e 15 de setembro e dias Ãºteis entre 4 de julho e 15 de setembro",
        120: "SÃ¡bados, domingos/feriados entre 10 de junho e 15 de setembro e dias Ãºteis de agosto",
        121: "SÃ¡bados, domingos/feriados entre 10 de junho e 15 de setembro e dias Ãºteis entre 1 de julho e 15 de setembro"
    }, YELLOW_CITIES = ["sintra", "amadora", "oeiras"], ORANGE_CITIES = ["almada", "sexial", "sesimbra"],
    GREEN_CITIES = ["marfa", "loures", "xira", "odivelas"],
    BLUE_CITIES = ["montijo", "moita", "alcochete", "palmela", "montijo2", "setubal"],
    GREY_CITIES = ["cascais", "lisboa", "barreiro"], YELLOW_DATA = {
        title: "Carris Metropolitana operada por ViaÃ§Ã£o Alvorada",
        duration: "",
        footer: [{
            text: "Idade media da frota no inicio",
            text2: "Menos de 7 meses"
        }, {text: "Remuneracao anual de referencia", text2: "56,7 MilhÃµes"}],
        schemes: [{label: "Oferta reforÃ§ada", num: "83"}, {label: "Total de linhas", num: "192"}, {
            label: "Novas",
            num: "41"
        }]
    }, ORANGE_DATA = {
        title: "Carris Metropolitana operada por Transportes Sul do Tejo",
        duration: "",
        footer: [{
            text: "Idade media da frota no inicio",
            text2: "Cerca de 11 meses"
        }, {text: "Remuneracao anual de referencia", text2: "37 MilhÃµes"}],
        schemes: [{label: "Oferta reforÃ§ada", num: "56"}, {label: "Total de linhas", num: "149"}, {
            label: "Novas",
            num: "50"
        }]
    }, GREEN_DATA = {
        title: "Carris Metropolitana operada por RodoviÃ¡ria de Lisboa",
        duration: "",
        footer: [{
            text: "Idade media da frota no inicio",
            text2: "Menos de 8 meses"
        }, {text: "Remuneracao anual de referencia", text2: "52,4 MilhÃµes"}],
        schemes: [{label: "Oferta reforÃ§ada", num: "134"}, {label: "Total de linhas", num: "327"}, {
            label: "Novas",
            num: "41"
        }]
    }, BLUE_DATA = {
        title: "Carris Metropolitana operada por Alsa Todi",
        duration: "",
        footer: [{text: "Idade media da frota no inicio", text2: "Novos"}, {
            text: "Remuneracao anual de referencia",
            text2: "26,4 MilhÃµes"
        }],
        schemes: [{label: "Oferta reforÃ§ada", num: "74"}, {label: "Total de linhas", num: "154"}, {
            label: "Novas",
            num: "30"
        }]
    }, GREY_DATA = {title: "", duration: "", footer: [], schemes: []},
    DATA = [YELLOW_CITIES.map((a => Object.assign({id: a}, YELLOW_DATA))), ORANGE_CITIES.map((a => Object.assign({id: a}, ORANGE_DATA))), GREEN_CITIES.map((a => Object.assign({id: a}, GREEN_DATA))), BLUE_CITIES.map((a => Object.assign({id: a}, BLUE_DATA))), GREY_CITIES.map((a => Object.assign({id: a}, GREY_DATA)))].flat(1);

function initSvgMap() {
    let a = document.documentElement;
    const o = document.querySelector("#svg-map");
    if (!o) return;
    const s = o.querySelectorAll(".segment");

    function i(a) {
        switch (a) {
            case 0:
                return "left";
            case 1:
                return "middle";
            case 2:
                return "right";
            case 3:
                return "bottom"
        }
    }

    function r(a, o, s) {
        const r = function (a) {
            const o = GREY_CITIES.includes(a.id), s = document.createElement("div");
            s.classList.add("tooltip");
            const r = document.createElement("div");
            r.classList.add("tooltip__header");
            const e = document.createElement("button");
            e.classList.add("tooltip__close-btn"), e.addEventListener("click", (function () {
                window.tippy.hideAll()
            }));
            const n = document.createElement("div");
            if (n.classList.add("tooltip__main-block"), n.appendChild(r), o) {
                r.appendChild(e), s.classList.add("js-missing-data");
                const a = document.createElement("div");
                a.classList.add("tooltip__missing-text"), a.innerText = "A OperaÃ§Ã£o da rede municipal no Barreiro, em Cascais e em Lisboa continua a ser realizada por operadores internos.", n.appendChild(a), s.appendChild(n)
            } else {
                const o = document.createElement("span");
                o.classList.add("tooltip__title"), o.innerText = a.title, r.appendChild(o), r.appendChild(e);
                const l = document.createElement("div");
                l.classList.add("tooltip__scheme");
                for (let o = 0; o < a.schemes.length; o++) {
                    const s = i(o), r = document.createElement("div"), e = document.createElement("b"),
                        n = document.createElement("span");
                    e.innerText = a.schemes[o].num, n.innerText = a.schemes[o].label, r.classList.add("scheme-item", "scheme-item--" + s), r.appendChild(e), r.appendChild(n), l.appendChild(r)
                }
                n.appendChild(l);
                const t = document.createElement("div");
                t.classList.add("tooltip__bottom-block");
                for (let o = 0; o < a.footer.length; o++) {
                    const s = document.createElement("div");
                    s.classList.add("tooltip__bottom-block__row");
                    const i = document.createElement("span"), r = document.createElement("b");
                    i.innerText = a.footer[o].text, r.innerText = a.footer[o].text2, s.appendChild(i), s.appendChild(r), t.appendChild(s)
                }
                s.appendChild(n), s.appendChild(t)
            }
            return s
        }(s);
        tippy(o, {
            allowHTML: !0,
            animation: "scale",
            arrow: !1,
            trigger: "click",
            triggerTarget: a,
            content: r,
            boundary: window,
            interactive: !0,
            appendTo: document.body
        })
    }

    function e(a) {
        return {tagColor: "#fff", segmentColor: "#eee", textColor: "#111", activeColor: a.rect[0].getAttribute("fill")}
    }

    function n(a, o) {
        a.elem.style.fill = o.activeColor, a.rect[0].style.fill = o.tagColor, a.textPath.style.fill = o.textColor
    }

    function l(a, o) {
        a.elem.style.fill = o.segmentColor, a.rect[0].style.fill = o.activeColor, a.textPath.style.fill = o.tagColor
    }

    function t() {
        o.querySelectorAll(".segment.js-active").forEach((function (a) {
            const o = a.getAttribute("data-target"), s = {
                rect: document.querySelectorAll("rect[data-target=" + o + "]"),
                textPath: document.querySelector("path[data-target=" + o + "]:not(.segment)"),
                elem: a
            }, i = e(s);
            a.classList.remove("js-active"), l(s, i)
        }))
    }

    s.forEach((function (o) {
        const s = o.getAttribute("data-target"), i = document.querySelectorAll("rect[data-target=" + s + "]"),
            d = {rect: i, textPath: document.querySelector("path[data-target=" + s + "]:not(.segment)"), elem: o},
            p = e(d), c = (u = s, DATA.find((a => a.id == u)));
        var u;
        !function (o, s, i) {
            o.elem.addEventListener("mouseover", (function () {
                n(o, s)
            })), o.elem.addEventListener("mouseout", (function () {
                o.elem.classList.contains("js-active") || l(o, s)
            })), o.elem.addEventListener("click", (function () {
                t(), o.elem.classList.add("js-active"), a.style.setProperty("--tt-active-color", s.activeColor), n(o, s), window.scrollIntoView(o.rect[0], {
                    behavior: "smooth",
                    scrollMode: "if-needed",
                    block: "nearest"
                })
            })), document.addEventListener("click", (function (a) {
                a.target.hasAttribute("data-target") || t()
            }))
        }(d, p), r(o, i[0], c)
    }));
    const d = window.outerWidth;
    if (d < 767) {
        const a = document.querySelector("#map-container"), o = document.querySelector("#map-inner-container"),
            s = +window.getComputedStyle(o).width.replace("px", "") / 2 - d / 2;
        a.scroll({top: 0, left: s, behavior: "auto"})
    }
}