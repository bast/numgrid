use rand::Rng;
use std::fmt::Debug;
use std::fs;
use std::num::ParseFloatError;
use std::str::FromStr;
use std::time::Instant;

fn floats_are_same(value: f64, reference: f64, threshold: f64) -> bool {
    let absolute_error = (value - reference).abs();
    if reference.abs() > threshold {
        let relative_error = (absolute_error / reference).abs();
        return relative_error < threshold;
    } else {
        return absolute_error < threshold;
    }
}

#[test]
fn angular_grid() {
    let (coordinates, weights) = numgrid::angular_grid(50);

    let coordinates_reference: [(f64, f64, f64); 50] = [
        (1.0, 0.0, 0.0),
        (-1.0, 0.0, 0.0),
        (0.0, 1.0, 0.0),
        (0.0, -1.0, 0.0),
        (0.0, 0.0, 1.0),
        (0.0, 0.0, -1.0),
        (0.0, 0.7071067811865475, 0.7071067811865475),
        (0.0, 0.7071067811865475, -0.7071067811865475),
        (0.0, -0.7071067811865475, 0.7071067811865475),
        (0.0, -0.7071067811865475, -0.7071067811865475),
        (0.7071067811865475, 0.0, 0.7071067811865475),
        (0.7071067811865475, 0.0, -0.7071067811865475),
        (-0.7071067811865475, 0.0, 0.7071067811865475),
        (-0.7071067811865475, 0.0, -0.7071067811865475),
        (0.7071067811865475, 0.7071067811865475, 0.0),
        (0.7071067811865475, -0.7071067811865475, 0.0),
        (-0.7071067811865475, 0.7071067811865475, 0.0),
        (-0.7071067811865475, -0.7071067811865475, 0.0),
        (0.5773502691896258, 0.5773502691896258, 0.5773502691896258),
        (0.5773502691896258, 0.5773502691896258, -0.5773502691896258),
        (0.5773502691896258, -0.5773502691896258, 0.5773502691896258),
        (0.5773502691896258, -0.5773502691896258, -0.5773502691896258),
        (-0.5773502691896258, 0.5773502691896258, 0.5773502691896258),
        (-0.5773502691896258, 0.5773502691896258, -0.5773502691896258),
        (-0.5773502691896258, -0.5773502691896258, 0.5773502691896258),
        (
            -0.5773502691896258,
            -0.5773502691896258,
            -0.5773502691896258,
        ),
        (0.3015113445777636, 0.3015113445777636, 0.9045340337332909),
        (0.3015113445777636, 0.3015113445777636, -0.9045340337332909),
        (0.3015113445777636, -0.3015113445777636, 0.9045340337332909),
        (0.3015113445777636, -0.3015113445777636, -0.9045340337332909),
        (-0.3015113445777636, 0.3015113445777636, 0.9045340337332909),
        (-0.3015113445777636, 0.3015113445777636, -0.9045340337332909),
        (-0.3015113445777636, -0.3015113445777636, 0.9045340337332909),
        (
            -0.3015113445777636,
            -0.3015113445777636,
            -0.9045340337332909,
        ),
        (0.3015113445777636, 0.9045340337332909, 0.3015113445777636),
        (0.3015113445777636, -0.9045340337332909, 0.3015113445777636),
        (0.3015113445777636, 0.9045340337332909, -0.3015113445777636),
        (0.3015113445777636, -0.9045340337332909, -0.3015113445777636),
        (-0.3015113445777636, 0.9045340337332909, 0.3015113445777636),
        (-0.3015113445777636, -0.9045340337332909, 0.3015113445777636),
        (-0.3015113445777636, 0.9045340337332909, -0.3015113445777636),
        (
            -0.3015113445777636,
            -0.9045340337332909,
            -0.3015113445777636,
        ),
        (0.9045340337332909, 0.3015113445777636, 0.3015113445777636),
        (-0.9045340337332909, 0.3015113445777636, 0.3015113445777636),
        (0.9045340337332909, 0.3015113445777636, -0.3015113445777636),
        (-0.9045340337332909, 0.3015113445777636, -0.3015113445777636),
        (0.9045340337332909, -0.3015113445777636, 0.3015113445777636),
        (-0.9045340337332909, -0.3015113445777636, 0.3015113445777636),
        (0.9045340337332909, -0.3015113445777636, -0.3015113445777636),
        (
            -0.9045340337332909,
            -0.3015113445777636,
            -0.3015113445777636,
        ),
    ];

    let weights_reference: [f64; 50] = [
        0.0126984126984127,
        0.0126984126984127,
        0.0126984126984127,
        0.0126984126984127,
        0.0126984126984127,
        0.0126984126984127,
        0.02257495590828924,
        0.02257495590828924,
        0.02257495590828924,
        0.02257495590828924,
        0.02257495590828924,
        0.02257495590828924,
        0.02257495590828924,
        0.02257495590828924,
        0.02257495590828924,
        0.02257495590828924,
        0.02257495590828924,
        0.02257495590828924,
        0.02109375,
        0.02109375,
        0.02109375,
        0.02109375,
        0.02109375,
        0.02109375,
        0.02109375,
        0.02109375,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
        0.02017333553791887,
    ];

    for (i, coordinate) in coordinates.iter().enumerate() {
        assert!(floats_are_same(
            coordinate.0,
            coordinates_reference[i].0,
            1.0e-15
        ));
        assert!(floats_are_same(
            coordinate.1,
            coordinates_reference[i].1,
            1.0e-15
        ));
        assert!(floats_are_same(
            coordinate.2,
            coordinates_reference[i].2,
            1.0e-15
        ));
    }

    for (i, &weight) in weights.iter().enumerate() {
        assert!(floats_are_same(weight, weights_reference[i], 1.0e-13));
    }
}

#[test]
fn radial_grid() {
    let alpha_min = vec![0.3023, 0.2753, 1.185];
    let (rs, ws) = numgrid::radial_grid(1.0e-12, &alpha_min, 11720.0, 2, 8);

    let rs_reference: [f64; 106] = [
        0.0000012304794589759454,
        0.0000026336980993310114,
        0.000004233905675553877,
        0.000006058756210858283,
        0.00000813978590080707,
        0.000010512958106784992,
        0.000013219284857637214,
        0.00001630553559996809,
        0.000019825045445384375,
        0.000023838636882425878,
        0.000028415670881768902,
        0.000033635245559400795,
        0.0000395875631124901,
        0.00004637548865068543,
        0.000054116327861820987,
        0.00006294385423279893,
        0.00007301062085911147,
        0.0000844905967945913,
        0.00009758217350152745,
        0.00011251159335718234,
        0.00012953685946652218,
        0.0001489521943486774,
        0.00017109312455002314,
        0.00019634227905373608,
        0.0002251360016911716,
        0.0002579718918275978,
        0.0002954174036368161,
        0.00033811965257322685,
        0.0003868165985121067,
        0.00044234979881978883,
        0.000505678951746213,
        0.000577898481471814,
        0.0006602564514236075,
        0.00075417613271137,
        0.000861280600419329,
        0.0009834207828147287,
        0.0011227074482063066,
        0.0012815476822342878,
        0.001462686485975001,
        0.0016692542137387164,
        0.0019048206703580484,
        0.002173456802850449,
        0.0024798050525806402,
        0.0028291595837154453,
        0.003227557774440908,
        0.003681884552049176,
        0.004199991374964317,
        0.004790831917897655,
        0.005464616804978631,
        0.006232990064885314,
        0.007109230357387858,
        0.008108480448805902,
        0.009248008902065284,
        0.010547508503756575,
        0.012029436585469348,
        0.013719403120673117,
        0.015646613304049737,
        0.01784437226172206,
        0.020350660614540016,
        0.023208790841033847,
        0.026468155782985957,
        0.030185082228936854,
        0.03442380432684787,
        0.03925757364797396,
        0.04476992508553923,
        0.05105612046487287,
        0.05822479481278606,
        0.06639983373623022,
        0.07572251435420017,
        0.0863539467814424,
        0.09847785835652628,
        0.11230376872997776,
        0.1280706106828306,
        0.1460508592488553,
        0.1665552404979834,
        0.1899381013558711,
        0.2166035332582641,
        0.24701235546623276,
        0.2816900787245926,
        0.32123598688766164,
        0.36633349345666244,
        0.41776195200551264,
        0.47641012459714155,
        0.5432915409450616,
        0.6195620137499113,
        0.7065396129026261,
        0.8057274437389025,
        0.9188396229878619,
        1.0478309013188452,
        1.1949304444090645,
        1.362680356320391,
        1.5539796109278552,
        1.7721341506018633,
        2.020914017925626,
        2.3046185077708192,
        2.6281504656586163,
        2.9971010163947933,
        3.417846187218798,
        3.897657095261897,
        4.444825603521343,
        5.068807616877284,
        5.780386494525543,
        6.591859402842075,
        7.517249829140259,
        8.572549928882209,
        9.775996894473037,
    ];

    let ws_reference: [f64; 106] = [
        0.0000000000000000019880914965294663,
        0.00000000000000001038653261137351,
        0.00000000000000003061060704476624,
        0.0000000000000000714837814384544,
        0.00000000000000014713551144970902,
        0.000000000000000279893223626772,
        0.0000000000000005046722390909259,
        0.0000000000000008756178543262147,
        0.0000000000000014761273623638226,
        0.0000000000000024339365998348335,
        0.000000000000003943784989150694,
        0.000000000000006301407768128589,
        0.000000000000009954441374268706,
        0.000000000000015578560677812623,
        0.000000000000024191229873523514,
        0.00000000000003732148809527238,
        0.00000000000005726316481952327,
        0.00000000000008745225174893691,
        0.0000000000001330289588817804,
        0.00000000000020167438677218996,
        0.0000000000003048554049444309,
        0.0000000000004596761382188584,
        0.0000000000006916306661132351,
        0.0000000000010386943244286718,
        0.0000000000015574028960741803,
        0.00000000000233188341812658,
        0.000000000003487266902098777,
        0.000000000005209605522315866,
        0.000000000007775443884457513,
        0.000000000011595717687661054,
        0.000000000017280913498749605,
        0.000000000025737776532522702,
        0.00000000003831282740110572,
        0.000000000057005327068056527,
        0.00000000008478327350554978,
        0.00000000012605224691926433,
        0.00000000018735099850206446,
        0.0000000002783833908343991,
        0.00000000041354926778222614,
        0.0000000006142153953807528,
        0.000000000912084137841181,
        0.0000000013541903530964091,
        0.0000000020103133079869633,
        0.0000000029839705608062017,
        0.000000004428724560632798,
        0.000000006572368889254967,
        0.000000009752801201202389,
        0.000000014471229140133474,
        0.000000021471083229777997,
        0.00000003185505615834441,
        0.000000047258687529971974,
        0.00000007010781105902893,
        0.00000010400037735496732,
        0.00000015427273619179805,
        0.0000002288394867408522,
        0.0000003394390493668513,
        0.0000005034809508773145,
        0.0000007467854907042957,
        0.0000011076468642926342,
        0.0000016428590532756463,
        0.000002436652612230245,
        0.0000036139484769927013,
        0.000005360014147918423,
        0.00000794961492856089,
        0.000011790246812106708,
        0.000017486253146137914,
        0.000025933912947518924,
        0.00003846246163719619,
        0.000057043231683500634,
        0.00008459981817680389,
        0.00012546806891829606,
        0.00018607827091049895,
        0.0002759668638565074,
        0.0004092768565180903,
        0.0006069830104367216,
        0.0009001918845830529,
        0.0013350359443087192,
        0.0019799317347829865,
        0.0029363440610608105,
        0.004354749722094859,
        0.006458312399005715,
        0.009577993503892579,
        0.014204623029315579,
        0.021066122775529127,
        0.031242031584589096,
        0.04633335097572609,
        0.06871443319306483,
        0.1019065414926407,
        0.15113185530871653,
        0.22413508250496608,
        0.33240194378869103,
        0.4929662346262761,
        0.7310897714237554,
        1.0842368603863297,
        1.6079685958458,
        2.3846843432564175,
        3.536585712027335,
        5.2449026512126045,
        7.77840652604825,
        11.535696161637674,
        17.10790975221833,
        25.371729152384425,
        37.62730899143959,
        55.80283111039023,
        82.75786703611777,
        122.73327675407306,
    ];

    for (&x, &x_reference) in rs.iter().zip(rs_reference.iter()) {
        assert!(floats_are_same(x, x_reference, 1.0e-15));
    }

    for (&x, &x_reference) in ws.iter().zip(ws_reference.iter()) {
        assert!(floats_are_same(x, x_reference, 1.0e-15));
    }
}

pub struct GridPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl FromStr for GridPoint {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split_whitespace().collect();

        let x = coords[0].parse::<f64>()?;
        let y = coords[1].parse::<f64>()?;
        let z = coords[2].parse::<f64>()?;
        let w = coords[3].parse::<f64>()?;

        Ok(GridPoint { x, y, z, w })
    }
}

fn read_vector<T: FromStr>(file_name: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let error_message = format!("something went wrong reading file {}", file_name);
    let contents = fs::read_to_string(file_name).expect(&error_message);
    let v = contents.lines().map(|s| s.parse().unwrap()).collect();

    return v;
}

#[test]
fn atom_grid() {
    let radial_precision = 1.0e-12;
    let alpha_min = vec![0.3023, 0.2753, 1.185];
    let alpha_max = 11720.0;
    let max_l_quantum_number = 2;
    let num_angular_points = 50;
    let num_centers = 1;
    let proton_charges = vec![8];
    let center_index = 0;
    let center_coordinates_bohr = vec![(0.0, 0.0, 0.0)];
    let hardness = 3;

    let (rs, ws) = numgrid::atom_grid(
        radial_precision,
        &alpha_min,
        alpha_max,
        max_l_quantum_number,
        num_angular_points,
        num_centers,
        &proton_charges,
        center_index,
        &center_coordinates_bohr,
        hardness,
    );

    let num_points = rs.len();
    assert_eq!(num_points, 5300);

    let reference_points: Vec<GridPoint> = read_vector("tests/reference/atom.txt");
    for (i, p) in reference_points.iter().enumerate() {
        assert!(floats_are_same(p.x, rs[i].0, 1.0e-15));
        assert!(floats_are_same(p.y, rs[i].1, 1.0e-15));
        assert!(floats_are_same(p.z, rs[i].2, 1.0e-15));
        assert!(floats_are_same(p.w, ws[i], 1.0e-15));
    }
}

#[test]
fn molecular_grid() {
    let radial_precision = 1.0e-12;
    let alpha_min = vec![
        vec![0.3023, 0.2753, 1.185],
        vec![0.122, 0.727],
        vec![0.122, 0.727],
    ];
    let alpha_max = vec![11720.0, 13.01, 13.01];
    let max_l_quantum_numbers = vec![2, 1, 1];
    let num_angular_points = 50;
    let num_centers = 3;
    let proton_charges = vec![8, 1, 1];
    let center_coordinates_bohr = vec![(0.0, 0.0, 0.0), (1.43, 0.0, 1.1), (-1.43, 0.0, 1.1)];
    let hardness = 3;

    let mut num_points = 0;
    let mut rs = Vec::new();
    let mut ws = Vec::new();
    for center_index in 0..num_centers {
        let (rs_atom, ws_atom) = numgrid::atom_grid(
            radial_precision,
            &alpha_min[center_index],
            alpha_max[center_index],
            max_l_quantum_numbers[center_index],
            num_angular_points,
            num_centers,
            &proton_charges,
            center_index,
            &center_coordinates_bohr,
            hardness,
        );
        num_points += rs_atom.len();
        rs.extend(rs_atom);
        ws.extend(ws_atom);
    }

    assert_eq!(num_points, 5300 + 3900 + 3900);

    let reference_points: Vec<GridPoint> = read_vector("tests/reference/molecule.txt");
    for (i, p) in reference_points.iter().enumerate() {
        // println!("{} {} {} {}", rs[i].0, rs[i].1, rs[i].2, ws[i]);
        assert!(floats_are_same(p.x, rs[i].0, 1.0e-15));
        assert!(floats_are_same(p.y, rs[i].1, 1.0e-15));
        assert!(floats_are_same(p.z, rs[i].2, 1.0e-15));
        assert!(floats_are_same(p.w, ws[i], 1.0e-15));
    }
}

#[ignore]
#[test]
fn benchmark() {
    let radial_precision = 1.0e-12;
    let num_angular_points = 50;
    let hardness = 3;

    let num_centers = 40;
    let proton_charges = vec![8; num_centers];

    let mut center_coordinates_bohr = Vec::new();

    let mut rng = rand::thread_rng();
    for _ in 0..num_centers {
        center_coordinates_bohr.push((
            rng.gen_range(-10.0, 10.0),
            rng.gen_range(-10.0, 10.0),
            rng.gen_range(-10.0, 10.0),
        ));
    }

    let start = Instant::now();
    for center_index in 0..num_centers {
        let (_rs_atom, _ws_atom) = numgrid::atom_grid(
            radial_precision,
            &vec![0.3023, 0.2753, 1.185],
            11720.0,
            2,
            num_angular_points,
            num_centers,
            &proton_charges,
            center_index,
            &center_coordinates_bohr,
            hardness,
        );
    }
    println!("time elapsed in molecular_grid: {:?}", start.elapsed());
}
