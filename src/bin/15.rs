use std::collections::BinaryHeap;
use std::cmp::Ordering;

static INPUT: &str = "9937913613489579689887899839718659949144483893874749858366681232699911196851919824197391598696712368\n5253918396598241962985986992428814875993889428717557939529588827685169588381194992989482737895729941\n4729891993659869351699195978189948897799346577498659736999269171689999888159836937748681169667242592\n9129275987879292235985782771919198489164694932897317998799366818498649189196178854891125199749869194\n9642428968649516499977253921652586978698849999177282414639999319775881939999474793794429627999449759\n3457847685152919578737716725398145788592567819997119869941598848979848884979739759929987549138674446\n1924959268497883988478799998486898967638863969799969998879997426283869935891217997963581576999477577\n9769699291551983797285999741971583335129491688689659175393852969839311784996989187976976117111744989\n8774389593493896722994716895989797954369988849881799329699172595587974241895375978516797413178898177\n7757596139726268841787827266693945489711877479699669162337887298659971765291991176598619988879458992\n9781789998158765594318599465891697957276431199651927962918499798242719599858458996999642258175758687\n8984938631984999991873832992988217977577978988159798986999178851998359989698487448999579673399774897\n1661397798982496194275298495879999794689914911178629732723288391847479896598583126788774879796265799\n9281148969889465884187756927991186475929999444177989898317769999164889399957969799586699849159525678\n9583368919996938196899472998382996697536979991982999365954516286975896779938891551819999868499439192\n4996699779179642972929187936871799218933962379999271194154831978198999687887833387197893787789989972\n2962954924185928529544742145794937851991889144889795199594977776328999977966299516948281759836422987\n4998979879489651798495821698919115968929995394999182184938529163541847116819993978985699688841679145\n1594337513971189196891612493989394833877688889499979954878958799694746895917899963959188484688859899\n9119535257779744999977197695152564898876941915262891977669997874469966996912689931815839999582919119\n9959891899989925394981283653289869995881919778578519992788492837984592677972998515489798198691154998\n9429989991658989899985969699996895998963789711997598184738138423864599295589767892235726256699928238\n9199962191939398598751959685292171988865194954999586968946299998212989147888686654847699389629469657\n7179336751985764599932791192256998929839671967294922699728149899633618695591698859191419451936788921\n8932785999367844386395984993659819299949897729979118972797819399875896479948952267467437948251283399\n9574659749394866921598198763778589997399487988289248763111793897822454955741541689189597889498857389\n8393896895573862969318814898967999878986946689899698199854629118991776697982989999849989891778876999\n4954898145734993349189384178422928781788977768283439213386938766971363822185788969989414694117881954\n7189979675893162998819596819896897379799928918928897169983989567656379896664969186989858472514989329\n5751997299544849917496939689718691685468629833352528678196299669458749997953846325943929942379679789\n3951419857745948546864498191466695969489926999977491477177873881878399683985898516791819213753299987\n9393954154111769116311642884815766487564788992443947999165887149815748894119986899939989557668927815\n7781949987885121991999882483818928342873947817382669815391655161819892179349683925953718649517396998\n9996948948421899596711727731449329968981947879892949998979527695699968594468673199919485662834387374\n2955857818438244239345917631963944664869385975973551395954867999958742182782929693999279119886989199\n1935779481368819399899295932749129919929122564789798977496958118274341968929935612899379896999434739\n9999379989796988993949894899672953947123999664969985992948759859991884717679943195999614481997469396\n8955859389791791697621795999799894697866973664816399131688499266949694993638918696719482969781831856\n7237599687234791197989283498113998773541471878751122891926938859888977925771628596359319722895885781\n1583298396196185914969785767692985979738116816939889946185188189996378425986548919999889997872812668\n7468729778772998838975721996464559897719967187379672699968443917729544944689797568925788267685298419\n8925761822513389919136399878898198998291175826111663968569779189296995869876694862793915816779979582\n8327728197898713986939892725495296392899935929887884748299886966918921779728919389894799897894889832\n6898942111437798616778962392231529964992899611983231149637299778948229977897769815928877679539394599\n8265295868197967186787681498843198771216977197955984841999943682984697973211892295875168788861917799\n8193655888847994164991829448191469651983896938799124981138989968916914294279468896713289981589259289\n3888577576615917661356499761814313745294798873992471474815359493988926667489369499984371126848817956\n4189688973398559698995585789979849869999988179674913199999948831138974727117999895794497185139599792\n8983995579523959889889238771849428955382191565219981899611744199249395936976858698348929739368496695\n8674818161989182767596969753979269697939995987921674346851383989197787935858899931675844397821638845\n6721526132582855278146961963994999516788241166588147297184495825982963899592149737997597782969196447\n8439719891564264896252269599969195695471974756859899818967897883591999984792735888817884128794539841\n8693988997995899177451568799489793996932941989991898887936681948764979628998487728594993391454678196\n7289188692759989919891981399965447981679194369535792557227848795987869847892333112691925883431991421\n9458159113799319212979988595449561197851898988976352781177919317585879183576919939448899969172979681\n1695989929956761839889616835346978282379197919881937992779896117389617929589968167953945893519916318\n3925158298861719396948534828519739165727495619961699893478891696889984568592872178788999499459895355\n2869917799798991249449813996589998588669414996394358576493271669859779111154919335687843929916299854\n9419881689895199119766456931857963399381516469374717659679967923871895999997758223399929495819819918\n7986939153649919376699996179783992794992784994897291914897348749588997322947915837494239139695441354\n8783499679318241615988127879859856962512129998487155997341866163754696716192574941999888979659561477\n8997696874937758149958889919716392587798298772495599898217574535443838899389934999268951618829167619\n9769921719999977952876429969119977869245637594832975591143859917316819949787823886562819988998819985\n5928986995414729762136357687574599449639993959619588985986848787992997838492819695425998997919475891\n6958739219929998833579576198988599789879558758653499992944796337268729483173669999615793193947962315\n7863789969384782631988853619618993911587993967419365692699729448881657929857848979736972789939937984\n6822964899911679534947781989456996499544749958985957128182117696611164767598998818212169566631818163\n5326666899284933914489819476949999689979919499789379867995685693658899285947799761164986742999279284\n8991268813171977922898185524764219887759979698134999497565771886689611992998169629277536173717461935\n5198432476687879698799521199937596782973371894616869219998149994662467229569899769791724585287589115\n1239158932851739871589128634979879899773725989792995917187777331193599799877259993465939819949936761\n3591979992998817938992164941999828898753979193159387999189899775759348387533743697198998893289688389\n6699412969998976549894874998887879878989589929121569939175893949656935991569963125185741637462667999\n8919788565416997139999747379772799995217121862796799286888938919183649327854643178359167982578711928\n6886444799381979182975398288191199268136569795947166379682838859216819664986249921898998753479791777\n1898197197719993689985896918658721975491689899859159917958958878177794825156667479929995752771429197\n6917973293689972689789611377276892993786638719767979299298645994759937959879653562926725796288833778\n4965459895377988942951916624989928829794427874967288967982148891714997585589388899483673492839896617\n8716555114983183171272797889895661993199133941191499673886499999138967995395595979991198989845611548\n5948843798979349299788217169916883629911839719948559688719332619182938919619835735942954876398194391\n3929879913379295283854897844783529816979887968492399917988987486971359257737381187569619989996957332\n8112496992139998489918755922199688199877913795559979759159142798595463879999495536858999972727869781\n4353646171197941653461749799976675878319798995579719199732583981621869798993761964978929186967892993\n9139263729235883969873989986142894184875958599992586658792977578142868729652825162298592988979122272\n9159971959419699919647649992829456619999798379519614435832739618178959479439975879797889673121685879\n1958989585992855779579572798476562982286672996789597896847342996239689571938918575965511979997864891\n9369887294599945787489188183796991686994757919997629984897996728962351189933887848696997899711959956\n1152721812994538878926991797969793998414877749156698644184744988619582157358154118791198877989968984\n5497584464939964859199199289999498959915768999817297993898597269296539886189618538788848465868999941\n6848663784529759999379968767595256682966361283981139937978986416795869573629997748299816595199898199\n9997198295279843127193982268516297267185245997238658783919717778819781895519922177783961979326866597\n3969895967193241993968744948671691945139688199737919787941997476793314792833895971983375298892979983\n9697629189525858769174819924269216981689591767858693855784649299975399499842761989439198989413412419\n8463279665993989749895994231581829192117893386229849898515917191989791111468959397963999996228583875\n9145589793868559193676799992674292794329699922137683537853816158451672577739744896992816568811397767\n9559733761815292969799878214141479794785963919313253976975511441729897165499643198779855984898228896\n8194986974737957997195785579939999987299969393696968868572189845878263991598581986947687899917929297\n6996263188158649929898298964287396897958491887329389178384868521218969959998297396994389797854411392\n7596539731575847988298699955435489839673789993939737916999898287181791248199961317979199871892989699\n6199797639684165538939589715319376942819972951831989171929899685175172299999133512854599293891739919";

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
  node: (usize,usize),
  cost: i32,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortes_path_bottom_corner(maze: &Vec<Vec<i32>>) -> i32 {
  let goal = (maze.len() -1, maze[0].len() - 1);
  let mut dist = vec![vec![i32::MAX; maze[0].len()]; maze.len()];
  let mut q = BinaryHeap::new();
  q.push(State {node: (0,0), cost: 0});
  while let Some(State { cost, node:(x,y) }) = q.pop() {
    if (x,y) == goal { return cost; }

    if cost > dist[x][y] { continue; }

    for (x1,y1) in [(x-1,y), (x+1,y), (x,y-1), (x,y+1)] {
      if maze.get(x1).and_then(|row| row.get(y1)).is_none() {
        continue;
      }
      let next = State { cost: cost + maze[x1][y1], node: (x1,y1) };
      if next.cost < dist[x1][y1] {
        q.push(next);
        dist[x1][y1] = next.cost;
      }
    }
  }
  0
}

aoc2021::main! {
  let maze = INPUT.lines()
    .map(|l| l.chars().map(|c| (c as u8 - b'0') as i32).collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let expanded = (0..(5*maze.len())).map(|x| (0..(5*maze[0].len()))
    .map(|y| {
      let xlevel = (x / maze.len()) as i32;
      let ylevel = (y / maze[0].len()) as i32;
      let cost = maze[x % maze.len()][y % maze[0].len()] + xlevel + ylevel;
      if cost < 10 {cost} else {cost - 9}
    })
    .collect::<Vec<_>>()
  )
  .collect::<Vec<_>>();
  let p1 = shortes_path_bottom_corner(&maze);
  let p2 = shortes_path_bottom_corner(&expanded);
  (p1,p2)
}
