/*

2000	16659,796	Idrico
2000	626,5	Geotermoelettrico
2000	6,307	Fotovoltaico
2000	363,435	Eolico
2000	671,202	Bioenergie
2001	16752,547	Idrico
2001	573	Geotermoelettrico
2001	6,565	Fotovoltaico
2001	663,855	Eolico
2001	684,568	Bioenergie
2002	16846,31	Idrico
2002	707	Geotermoelettrico
2002	6,41	Fotovoltaico
2002	780,11	Eolico
2002	891,736	Bioenergie
2003	16995,5485	Idrico
2003	707	Geotermoelettrico
2003	7,042	Fotovoltaico
2003	873,64	Eolico
2003	1086,475	Bioenergie
2004	17084,2	Idrico
2004	681,0001	Geotermoelettrico
2004	7,124	Fotovoltaico
2004	1131,485	Eolico
2004	1191,869	Bioenergie
2005	17348,773	Idrico
2005	711	Geotermoelettrico
2005	7,124	Fotovoltaico
2005	1638,955	Eolico
2005	1194,85	Bioenergie
2006	17440,51	Idrico
2006	711	Geotermoelettrico
2006	7,17381	Fotovoltaico
2006	1908,287	Eolico
2006	1255,525	Bioenergie
2007	17458,614	Idrico
2007	711	Geotermoelettrico
2007	88,14027	Fotovoltaico
2007	2714,128	Eolico
2007	1336,882	Bioenergie
2008	17651,245	Idrico
2008	711	Geotermoelettrico
2008	431,9322	Fotovoltaico
2008	3537,578	Eolico
2008	1555,342	Bioenergie
2009	17750,035	Idrico
2009	737	Geotermoelettrico
2009	1142,19014	Fotovoltaico
2009	4897,938	Eolico
2009	2018,554	Bioenergie
2010	17903,71	Idrico
2010	772	Geotermoelettrico
2010	3469,885	Fotovoltaico
2010	5814,281	Eolico
2010	2351,545	Bioenergie
2011	18119,83891	Idrico
2011	772	Geotermoelettrico
2011	12773,374	Fotovoltaico
2011	6936,14619	Eolico
2011	2825,3295	Bioenergie
2012	18259,38741	Idrico
2012	772	Geotermoelettrico
2012	16419,861	Fotovoltaico
2012	8119,40131	Eolico
2012	3801,5725	Bioenergie
2013	18393,18122	Idrico
2013	772,99	Geotermoelettrico
2013	18185,465	Fotovoltaico
2013	8560,80791	Eolico
2013	4033,4215	Bioenergie
2014	18421,27242	Idrico
2014	820,99	Geotermoelettrico
2014	18594,377	Fotovoltaico
2014	8703,077	Eolico
2014	4043,6357	Bioenergie
2015	18542,61953	Idrico
2015	820,99	Geotermoelettrico
2015	18900,78995	Fotovoltaico
2015	9161,944	Eolico
2015	4056,5367	Bioenergie
2016	18640,79953	Idrico
2016	814,59	Geotermoelettrico
2016	19283,17269	Fotovoltaico
2016	9409,934	Eolico
2016	4124,0797	Bioenergie
2017	18862,92513	Idrico
2017	813,09	Geotermoelettrico
2017	19682,293	Fotovoltaico
2017	9765,856	Eolico
2017	4135,0337	Bioenergie
2018	18935,50703	Idrico
2018	813,09	Geotermoelettrico
2018	20107,588	Fotovoltaico
2018	10264,69	Eolico
2018	4180,3955	Bioenergie
2019	18982,33241	Idrico
2019	813,09	Geotermoelettrico
2019	20865,275	Fotovoltaico
2019	10714,754	Eolico
2019	4119,7405	Bioenergie
2020	19105,91042	Idrico
2020	817,09	Geotermoelettrico
2020	21650,035	Fotovoltaico
2020	10906,856	Eolico
2020	4105,9305	Bioenergie
2021	19172,26241	Idrico
2021	817,09	Geotermoelettrico
2021	22594,259	Fotovoltaico
2021	11289,805	Eolico
2021	4105,8495	Bioenergie
2022	19265,28943	Idrico
2022	817,09	Geotermoelettrico
2022	25063,919	Fotovoltaico
2022	11858,43	Eolico
2022	4049,4585	Bioenergie
2023	19274,1695	Idrico
2023	817,09	Geotermoelettrico
2023	30319,417	Fotovoltaico
2023	12335,543	Eolico
2023	4078,76	Bioenergie

*/

pub struct DatoStorico {
    pub anno: Vec<i32>,
    pub idrico: Vec<f64>,
    pub geotermoelettrico: Vec<f64>,
    pub fotovoltaico: Vec<f64>,
    pub eolico: Vec<f64>,
    pub bioenergie: Vec<f64>,
    pub termoelettrico: Vec<f64>,
}

pub fn get_potenza_installata() -> DatoStorico {
    DatoStorico {
        anno: vec![
            2000, 2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011, 2012, 2013,
            2014, 2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023,
        ],
        idrico: vec![
            16659.796,
            16752.547,
            16846.31,
            16995.5485,
            17084.2,
            17348.773,
            17440.51,
            17458.614,
            17651.245,
            17750.035,
            17903.71,
            18119.83891,
            18259.38741,
            18393.18122,
            18421.27242,
            18542.61953,
            18640.79953,
            18862.92513,
            18935.50703,
            18982.33241,
            19105.91042,
            19172.26241,
            19265.28943,
            19274.1695,
        ],
        geotermoelettrico: vec![
            626.5, 573.0, 707.0, 707.0, 681.0001, 711.0, 711.0, 711.0, 711.0, 737.0, 772.0, 772.0,
            772.0, 772.99, 820.99, 820.99, 814.59, 813.09, 813.09, 813.09, 817.09, 817.09, 817.09,
            817.09,
        ],
        fotovoltaico: vec![
            6.307,
            6.565,
            6.41,
            7.042,
            7.124,
            7.124,
            7.17381,
            88.14027,
            431.9322,
            1142.19014,
            3469.885,
            12773.374,
            16419.861,
            18185.465,
            18594.377,
            18900.78995,
            19283.17269,
            19682.293,
            20107.588,
            20865.275,
            21650.035,
            22594.259,
            25063.919,
            30319.417,
        ],
        eolico: vec![
            363.435, 663.855, 780.11, 873.64, 1131.485, 1638.955, 1908.287, 2714.128, 3537.578,
            4897.938, 5814.281, 6936.14619, 8119.40131, 8560.80791, 8703.077, 9161.944, 9409.934,
            9765.856, 10264.69, 10714.754, 10906.856, 11289.805, 11858.43, 12335.543,
        ],
        bioenergie: vec![
            671.202, 684.568, 891.736, 1086.475, 1191.869, 1194.85, 1255.525, 1336.882, 1555.342,
            2018.554, 2351.545, 2825.3295, 3801.5725, 4033.4215, 4043.6357, 4056.5367, 4124.0797,
            4135.0337, 4180.3955, 4119.7405, 4105.9305, 4105.8495, 4049.4585, 4078.76,
        ],
        termoelettrico: vec![],
    }
}

/*
// produzione storico

2000	220454,894588	Termoelettrico
2000	50899,6346976	Idrico
2000	4705,19142	Geotermoelettrico
2000	563,072892	Eolico
2000	6,293099	Fotovoltaico
2001	219378,871936	Termoelettrico
2001	53925,656052	Idrico
2001	4506,56922	Geotermoelettrico
2001	1178,591242	Eolico
2001	4,847992	Fotovoltaico
2002	231068,661591	Termoelettrico
2002	47262,011356	Idrico
2002	4662,2729	Geotermoelettrico
2002	1404,233384	Eolico
2002	4,079782	Fotovoltaico
2003	242784,35953	Termoelettrico
2003	44276,765847	Idrico
2003	5340,46247	Geotermoelettrico
2003	1458,447703	Eolico
2003	5,011693	Fotovoltaico
2004	246125,309723	Termoelettrico
2004	49908,0054898	Idrico
2004	5437,26656	Geotermoelettrico
2004	1846,544201	Eolico
2004	4,041728	Fotovoltaico
2005	253073,122835	Termoelettrico
2005	42926,900784	Idrico
2005	5324,46041	Geotermoelettrico
2005	2343,399702	Eolico
2005	3,988234	Fotovoltaico
2006	262164,900336	Termoelettrico
2006	43424,963691	Idrico
2006	5527,36588	Geotermoelettrico
2006	2970,735834	Eolico
2006	2,294292	Fotovoltaico
2007	265764,207739	Termoelettrico
2007	38481,3489141	Idrico
2007	5569,12819	Geotermoelettrico
2007	4034,3591215	Eolico
2007	38,95327408	Fotovoltaico
2008	261328,430002	Termoelettrico
2008	47226,537568	Idrico
2008	5520,31462	Geotermoelettrico
2008	4861,317233	Eolico
2008	192,96499317	Fotovoltaico
2009	226637,893734	Termoelettrico
2009	53442,67806	Idrico
2009	6542,8587168	Eolico
2009	5341,82207	Geotermoelettrico
2009	676,48053728	Fotovoltaico
2010	231248,044894	Termoelettrico
2010	54406,662891	Idrico
2010	9125,91850442	Eolico
2010	5375,91565	Geotermoelettrico
2010	1905,660774	Fotovoltaico
2011	228506,619306	Termoelettrico
2011	47756,917968	Idrico
2011	10795,723458	Fotovoltaico
2011	9856,375161	Eolico
2011	5654,26305	Geotermoelettrico
2012	217561,405071	Termoelettrico
2012	43854,000715	Idrico
2012	18861,731562	Fotovoltaico
2012	13407,126958	Eolico
2012	5591,68511	Geotermoelettrico
2013	192986,752767	Termoelettrico
2013	54671,59394349	Idrico
2013	21588,62212	Fotovoltaico
2013	14896,962326	Eolico
2013	5659,2279	Geotermoelettrico
2014	176171,198784	Termoelettrico
2014	60256,343573	Idrico
2014	22306,364693	Fotovoltaico
2014	15178,310262	Eolico
2014	5916,33255	Geotermoelettrico
2015	192053,507174	Termoelettrico
2015	46969,4679932	Idrico
2015	22942,192097	Fotovoltaico
2015	14843,88246625	Eolico
2015	6184,97062	Geotermoelettrico
2016	199429,7324553	Termoelettrico
2016	44256,96018	Idrico
2016	22104,258953	Fotovoltaico
2016	17688,669481	Eolico
2016	6288,57344	Geotermoelettrico
2017	209484,560548	Termoelettrico
2017	38024,671209	Idrico
2017	24377,711329	Fotovoltaico
2017	17741,90871	Eolico
2017	6201,15891	Geotermoelettrico
2018	192730,019948	Termoelettrico
2018	50502,7514221	Idrico
2018	22653,838019	Fotovoltaico
2018	17716,4337258	Eolico
2018	6105,39009	Geotermoelettrico
2019	195733,8875205	Termoelettrico
2019	48153,515727	Idrico
2019	23688,900488	Fotovoltaico
2019	20202,041423	Eolico
2019	6074,86042	Geotermoelettrico
2020	181306,5868985	Termoelettrico
2020	49495,2547913	Idrico
2020	24941,50373	Fotovoltaico
2020	18761,55628667	Eolico
2020	6026,11251	Geotermoelettrico
2021	189711,085682	Termoelettrico
2021	47478,36015355	Idrico
2021	25038,989734	Fotovoltaico
2021	20927,30287472	Eolico
2021	5913,78727	Geotermoelettrico
2022	199209,7317928	Termoelettrico
2022	30290,7428966	Idrico
2022	28121,457534	Fotovoltaico
2022	20494,17807691	Eolico
2022	5836,92032	Geotermoelettrico
2023	162587,72499309	Termoelettrico
2023	42068,25964395	Idrico
2023	30711,082118	Fotovoltaico
2023	23640,47162144	Eolico
2023	5700,553644	Geotermoelettrico

*/

pub fn get_produzione() -> DatoStorico {
    DatoStorico {
        anno: vec![
            2000, 2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011, 2012, 2013,
            2014, 2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023,
        ],
        idrico: vec![
            50899.6346976,
            53925.656052,
            47262.011356,
            44276.765847,
            49908.0054898,
            42926.900784,
            43424.963691,
            38481.3489141,
            47226.537568,
            53442.67806,
            54406.662891,
            47756.917968,
            43854.000715,
            54671.59394349,
            60256.343573,
            46969.4679932,
            44256.96018,
            38024.671209,
            50502.7514221,
            48153.515727,
            49495.2547913,
            47478.36015355,
            30290.7428966,
            42068.25964395,
        ],
        geotermoelettrico: vec![
            4705.19142,
            4506.56922,
            4662.2729,
            5340.46247,
            5437.26656,
            5324.46041,
            5527.36588,
            5569.12819,
            5520.31462,
            5341.82207,
            5375.91565,
            5654.26305,
            5591.68511,
            5659.2279,
            5916.33255,
            6184.97062,
            6288.57344,
            6201.15891,
            6105.39009,
            6074.86042,
            6026.11251,
            5913.78727,
            5836.92032,
            5700.553644,
        ],
        fotovoltaico: vec![
            6.293099,
            4.847992,
            4.079782,
            5.011693,
            4.041728,
            3.988234,
            2.294292,
            38.95327408,
            192.96499317,
            676.48053728,
            1905.660774,
            10795.723458,
            18861.731562,
            21588.62212,
            22306.364693,
            22942.192097,
            22104.258953,
            24377.711329,
            22653.838019,
            23688.900488,
            24941.50373,
            25038.989734,
            28121.457534,
            30711.082118,
        ],
        eolico: vec![
            563.072892,
            1178.591242,
            1404.233384,
            1458.447703,
            1846.544201,
            2343.399702,
            2970.735834,
            4034.3591215,
            4861.317233,
            6542.8587168,
            9125.91850442,
            9856.375161,
            13407.126958,
            14896.962326,
            15178.310262,
            14843.88246625,
            17688.669481,
            17741.90871,
            17716.4337258,
            20202.041423,
            18761.55628667,
            20927.30287472,
            20494.17807691,
            23640.47162144,
        ],
        bioenergie: vec![],
        termoelettrico: vec![
            220454.894588,
            219378.871936,
            231068.661591,
            242784.35953,
            246125.309723,
            253073.122835,
            262164.900336,
            265764.207739,
            261328.430002,
            226637.893734,
            231248.044894,
            228506.619306,
            217561.405071,
            192986.752767,
            176171.198784,
            199429.7324553,
            192730.019948,
            195733.8875205,
            181306.5868985,
            189711.085682,
            199209.7317928,
            162587.72499309,
        ],
    }
}
