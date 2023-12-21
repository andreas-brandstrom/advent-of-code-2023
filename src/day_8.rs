use std::{collections::HashMap, char};

type NodeDirections<'a> = (&'a [char; 3], &'a([char; 3], [char; 3]));

pub(crate) fn haunted_wastland() {
    let data = 
        "LRLRLLRLRLRRLRLRLRRLRLRLLRRLRRLRLRLRLLRRRLRRRLLRRLRLRLRRRLRRLRRRLRLRLRRLRLLRLRLRRLRRRLRLRRLRRRLLRLRLRRRLRRRLRLRRRLRLRRRLLRRLLLRRRLLRRRLRRRLRRRLRLRLRLLRLRRLRLRLLLRRLRRLRRLRLRRLRRLLRRLRLRRRLRLRLLRRRLRRRLRRRLLLRRRLRLRLRRLRRRLRRRLRLRRRLRRLRRRLRLRRLLRRRLRRRLLLRRLRLRLRRLRRRLRRLRRLRLRRRR

        GXT = (MQM, CHN)
        MBK = (RCK, RCK)
        HBS = (QHS, RXC)
        SXK = (FDB, FKP)
        NJB = (BSB, KJM)
        SPD = (FNL, RSH)
        FJF = (NFH, XJN)
        GHV = (LSV, BTS)
        QDT = (HXV, PDX)
        MDH = (XDK, DKN)
        AAA = (FKL, CFC)
        GRB = (VDP, LMM)
        CXK = (DVB, CRJ)
        FDB = (FTD, CNK)
        LQT = (BJV, SMQ)
        TSK = (NQD, VSG)
        VLF = (NDS, CTV)
        PGP = (DKC, CKL)
        PVJ = (FDB, FKP)
        VSV = (NFP, QHX)
        KXN = (XJN, NFH)
        KMQ = (VBH, XXH)
        QXR = (RMD, TLT)
        DLN = (TPD, KBG)
        BHK = (GRP, RXF)
        TSX = (HQP, SHK)
        PTV = (VSG, NQD)
        QVN = (XBH, DHC)
        DDM = (TCB, XRQ)
        NKD = (CDR, BJM)
        JNR = (FMC, SQN)
        VPQ = (JGC, VCJ)
        HPB = (STQ, DDM)
        HRT = (JNR, BGH)
        CNQ = (HQV, PJQ)
        PMG = (LRB, XXP)
        RKV = (XGN, VCG)
        KVQ = (KHS, SLV)
        MDM = (VDX, NSF)
        VHT = (PGP, GJS)
        BPD = (NBF, VNH)
        JCQ = (JCB, XVR)
        CFJ = (PQP, CBJ)
        DSX = (BXN, VDS)
        MGH = (PFV, NLQ)
        MPK = (FND, BJX)
        QFR = (HFC, CNG)
        PHS = (VNH, NBF)
        KTT = (MTG, SQM)
        JBK = (CSR, VXV)
        BKL = (DLB, SHQ)
        GQH = (PCH, LDZ)
        XVJ = (CQM, SLF)
        VBN = (FFF, PDL)
        KQH = (SLT, XLG)
        SSN = (PQH, LFT)
        MQM = (SBH, TTC)
        SCR = (HGH, QGS)
        XTZ = (XKM, JTJ)
        ZZZ = (CFC, FKL)
        PRA = (GRB, MDB)
        VTV = (LXQ, HKP)
        PPX = (SQM, MTG)
        PVA = (MRJ, CVH)
        BJH = (VTR, RKG)
        KPH = (RFD, NJP)
        HXG = (NVJ, HNG)
        LRX = (DCX, MGH)
        NFD = (PFR, FVQ)
        TDG = (TDH, ZZZ)
        XKM = (XDG, VHT)
        CKT = (LDC, VPF)
        QQR = (BFQ, FXC)
        RGJ = (KTV, GFR)
        XDK = (MVC, SPX)
        TLT = (MBK, DGL)
        CDR = (JBG, KPH)
        LTH = (DFN, BHK)
        PVV = (BFQ, FXC)
        FCK = (FRJ, KVG)
        FXB = (MDH, VSJ)
        DHD = (LDJ, RNH)
        HHR = (HSK, CJD)
        LSV = (LNP, NMD)
        JSX = (DPF, SNN)
        SBH = (XKD, GTX)
        BHJ = (JSX, TMN)
        CQX = (XPG, RLB)
        XRQ = (FJK, NCF)
        GMF = (JBX, DRV)
        KBD = (HCC, TND)
        KGT = (NLK, FRX)
        RKM = (RTM, FMQ)
        GNM = (GML, NLB)
        SXP = (RKG, VTR)
        PMS = (XBH, DHC)
        PQH = (PJD, CXK)
        XXH = (BMB, JGJ)
        RHL = (QHS, RXC)
        DFS = (JVK, VBN)
        QCS = (XGN, VCG)
        MJJ = (DDM, STQ)
        SPV = (PXX, RRT)
        FRJ = (SNX, MPQ)
        FKC = (CBF, RKM)
        SRG = (QFH, LGP)
        VVQ = (BJV, SMQ)
        BQC = (DSP, TXQ)
        SHQ = (NKR, RGJ)
        KCS = (QGS, HGH)
        MTV = (BXN, VDS)
        CNC = (HPB, MJJ)
        SNF = (XXP, LRB)
        KTP = (QRX, KML)
        GSC = (HSK, CJD)
        QRX = (CRB, TXL)
        MDB = (VDP, LMM)
        PFV = (CLM, CTN)
        RRB = (HHR, GSC)
        DDQ = (PXX, RRT)
        TJB = (LSX, XNR)
        BSP = (TXC, LKB)
        MMV = (DKK, NFD)
        DCB = (XKP, TNJ)
        QKV = (CTT, SFM)
        XDG = (PGP, GJS)
        JMJ = (JLK, NJB)
        VBJ = (HJV, JFD)
        QHM = (CLJ, QGP)
        STR = (HXG, RFC)
        DPF = (VNQ, DQQ)
        RXF = (NHV, GBX)
        FXC = (JLX, LBS)
        XVR = (MVT, QHD)
        KNM = (BJD, QCK)
        GTX = (MSB, FNP)
        XCS = (PTN, NRH)
        CGS = (FRX, NLK)
        RCK = (CQM, CQM)
        CDS = (PRR, GCJ)
        BXN = (KQC, MSV)
        PTN = (VST, QHJ)
        TPD = (HJF, TSS)
        RNH = (TTK, DBN)
        TSM = (TPD, KBG)
        KTG = (GPT, BMD)
        PJP = (XBR, TCM)
        QPF = (KJF, NJX)
        HKP = (PLS, PGK)
        KBG = (HJF, TSS)
        SQK = (LSD, KNM)
        XKD = (MSB, FNP)
        QQP = (LFB, LTK)
        MDX = (KTJ, FXF)
        JLK = (KJM, BSB)
        FBR = (PMG, SNF)
        GSJ = (TDH, TDH)
        DQQ = (FKK, KRF)
        CTV = (CQD, RQX)
        MNT = (RLG, XXQ)
        GFL = (FGJ, KMD)
        BJK = (VQX, JCQ)
        SPN = (NKS, CNQ)
        LNL = (XLM, XLC)
        HKF = (HKJ, NNC)
        FTD = (CCX, MSS)
        RPV = (MDQ, DGS)
        DCX = (NLQ, PFV)
        JGB = (DVT, DRH)
        CGM = (CHN, MQM)
        JNQ = (PPX, KTT)
        BJM = (JBG, KPH)
        NFP = (SPD, XCT)
        TPL = (LSD, KNM)
        XTV = (CTJ, HPX)
        LKB = (PDF, SLK)
        QVK = (KMQ, MGF)
        QVP = (VCX, JNQ)
        NSR = (TJB, NBB)
        XLA = (DLB, SHQ)
        QLP = (HBS, RHL)
        BGH = (FMC, SQN)
        BJX = (LRX, PNM)
        MVC = (BPD, PHS)
        DXQ = (TMN, JSX)
        PBK = (QKV, CTH)
        GBJ = (RNH, LDJ)
        BMB = (HJS, TJT)
        SHT = (MFQ, LCT)
        HSK = (CGS, KGT)
        DRH = (PKN, PBN)
        PQF = (XKP, TNJ)
        RPX = (HCT, HQT)
        FPH = (VQL, NPJ)
        GLC = (PMS, QVN)
        VDP = (SXK, PVJ)
        JFD = (QBD, DGT)
        LQL = (NPT, GVS)
        TPQ = (NHL, MTD)
        KRF = (HVT, MPK)
        JQG = (QDT, PCK)
        PJQ = (KVH, LBV)
        VBB = (XXQ, RLG)
        PSH = (SQV, TLB)
        VXH = (VCX, JNQ)
        VQD = (NBB, TJB)
        JVD = (GDJ, JGQ)
        SDR = (FTF, NQL)
        NQQ = (TFM, CMC)
        FNL = (LTV, CSX)
        FVS = (NSF, VDX)
        VTJ = (VXH, QVP)
        KQC = (LGG, LGG)
        SCS = (MTV, DSX)
        QPS = (HMV, QQP)
        CQM = (JDD, JDD)
        FJX = (VBB, MNT)
        GBQ = (BXV, LFR)
        HGH = (SQK, TPL)
        SLT = (FXB, NGN)
        PBD = (HDC, FCX)
        CKL = (MQS, NND)
        LSX = (MBF, SPN)
        HNG = (JQG, GKD)
        MFQ = (PSH, TGK)
        SNX = (GCS, KSF)
        FMQ = (DXT, DFS)
        TXC = (SLK, PDF)
        VST = (SSN, LSR)
        BFB = (VSV, SVC)
        VNX = (TMF, QFS)
        VPK = (VBB, MNT)
        NVR = (XPG, RLB)
        FLS = (FSL, CKH)
        FKP = (FTD, CNK)
        MGF = (XXH, VBH)
        LNM = (NFD, DKK)
        FNP = (VTT, MVG)
        HDC = (PXP, GVH)
        KVL = (LCT, MFQ)
        XGP = (RHG, RTK)
        JCT = (NJB, JLK)
        XSL = (SVC, VSV)
        SRS = (HPD, XBL)
        FSL = (JMG, TFD)
        CXB = (KQV, QGK)
        QRP = (CGF, CQF)
        PRG = (MDX, KPL)
        VLL = (MRB, MHN)
        VNQ = (KRF, FKK)
        NPJ = (VPK, FJX)
        MHN = (SRG, TTL)
        KFN = (DFN, BHK)
        FVQ = (LJM, TJV)
        GKD = (QDT, PCK)
        GVS = (FCK, NGC)
        MBF = (CNQ, NKS)
        SMQ = (KSV, KMF)
        CRJ = (GSJ, TDG)
        LHT = (BTL, KMB)
        KJM = (KJC, BGC)
        TTC = (GTX, XKD)
        QFT = (PRM, KXS)
        SDS = (JMP, TSR)
        BLX = (PQF, DCB)
        PXX = (LJT, QFK)
        SQV = (FCB, HXF)
        RQX = (DBJ, VTV)
        THV = (GJL, MND)
        SLK = (XBP, CXB)
        LVS = (CTJ, HPX)
        TSS = (CGL, JST)
        MTG = (RKT, GTJ)
        FFF = (GBQ, VTM)
        XMV = (KVL, SHT)
        FDJ = (MLG, MLG)
        HPD = (MQK, FLS)
        TLB = (HXF, FCB)
        BGC = (SCR, KCS)
        FBM = (PBD, MFK)
        TJP = (JSS, GHV)
        TXL = (NQC, RXH)
        TKM = (GLQ, TKP)
        RNS = (BJK, PHJ)
        RRT = (LJT, QFK)
        DHC = (XVF, STR)
        RXM = (KVL, SHT)
        JBX = (GBJ, DHD)
        TBR = (HJV, JFD)
        MFK = (HDC, FCX)
        TFD = (QXN, QSK)
        HMV = (LFB, LTK)
        RMD = (MBK, DGL)
        RJP = (CTX, TPQ)
        CBF = (FMQ, RTM)
        XXG = (JDH, TJP)
        PDX = (XGQ, BSP)
        XBL = (MQK, FLS)
        GMT = (PCB, GTK)
        DGT = (GNH, KVQ)
        CHN = (TTC, SBH)
        NKS = (HQV, PJQ)
        HJC = (DXQ, BHJ)
        CSR = (TSX, HSM)
        XTK = (BJM, CDR)
        DRT = (MPB, PSV)
        HQT = (CRM, XTZ)
        HNM = (SXJ, GMF)
        NLQ = (CLM, CTN)
        LTK = (PGS, SCS)
        DNR = (DRT, QLK)
        QVR = (DLN, TSM)
        VLD = (GSP, CKT)
        LFT = (PJD, CXK)
        KVG = (SNX, MPQ)
        FBS = (JSN, KFT)
        GTK = (XHH, LCL)
        KJZ = (MDB, GRB)
        QGK = (PKK, VFK)
        VDS = (KQC, MSV)
        KHS = (TRF, JVD)
        BHH = (MLG, RPX)
        JVL = (CQF, CGF)
        RGQ = (CQJ, KTG)
        VTM = (LFR, BXV)
        GML = (VFF, VLD)
        CTJ = (FKC, PDT)
        BJV = (KSV, KSV)
        SPX = (BPD, PHS)
        CTX = (NHL, MTD)
        LRB = (HHP, CDS)
        XHH = (MKX, KQH)
        FDL = (QFT, MNG)
        CCX = (HKF, VPS)
        GPT = (VHX, QPF)
        VFK = (VJX, QPS)
        QFH = (LHP, CKD)
        KMD = (QFR, XVX)
        MSB = (VTT, MVG)
        TRF = (JGQ, GDJ)
        PHJ = (VQX, JCQ)
        DBN = (NRQ, GBN)
        JPM = (PMG, SNF)
        TND = (HQB, XXT)
        FJK = (XHV, FBS)
        PNM = (DCX, MGH)
        QFK = (RKR, QVK)
        VNV = (NRC, XXN)
        QXN = (PSG, CFJ)
        NND = (HJC, MPN)
        DQM = (TJP, JDH)
        PGK = (XQK, PRG)
        HMK = (BJK, PHJ)
        DLB = (RGJ, NKR)
        PXP = (XTK, NKD)
        LGG = (CNP, CNP)
        HKJ = (VQD, NSR)
        TMN = (SNN, DPF)
        FRX = (FBR, JPM)
        PTA = (JTJ, XKM)
        NRH = (VST, QHJ)
        BQD = (KTP, GFF)
        VTT = (KFN, LTH)
        GMV = (VCJ, JGC)
        XLG = (FXB, NGN)
        JDH = (GHV, JSS)
        TJT = (JCT, JMJ)
        HCT = (CRM, CRM)
        BGR = (JMP, TSR)
        CKH = (JMG, TFD)
        DHX = (QGP, CLJ)
        BQN = (TBN, BTQ)
        RSH = (CSX, LTV)
        SXJ = (DRV, JBX)
        MRB = (SRG, TTL)
        GTG = (SPV, DDQ)
        VQL = (VPK, FJX)
        RXC = (GXT, CGM)
        MSV = (LGG, HLF)
        PSV = (GBL, RRB)
        LRV = (XTV, LVS)
        MVR = (NQL, FTF)
        JDD = (GRB, MDB)
        TKG = (LHT, VLM)
        NHL = (MMV, LNM)
        XBR = (DHX, QHM)
        DNK = (TXQ, DSP)
        GBX = (SDS, BGR)
        PDT = (CBF, RKM)
        PGH = (FGJ, KMD)
        GTR = (RPT, CXM)
        GBP = (LNB, DNR)
        DKC = (MQS, NND)
        HSM = (SHK, HQP)
        VVV = (GLQ, TKP)
        TTJ = (DNK, BQC)
        CJD = (CGS, KGT)
        VSG = (RMJ, BQN)
        XGQ = (TXC, LKB)
        CNP = (PCH, PCH)
        JPT = (NDS, CTV)
        MVT = (GBP, RLR)
        HJF = (CGL, JST)
        KGB = (GML, NLB)
        MPN = (DXQ, BHJ)
        KFT = (DTS, DJP)
        QCK = (RLF, QVR)
        MJR = (MJD, RPS)
        LBV = (FPH, RFH)
        JMG = (QSK, QXN)
        LDJ = (DBN, TTK)
        XBP = (KQV, QGK)
        RTM = (DFS, DXT)
        VBH = (BMB, JGJ)
        LHP = (MMH, BPV)
        PMR = (MDQ, DGS)
        NNR = (JDF, NVP)
        GJL = (JDT, QLP)
        JST = (SBL, LNL)
        KML = (TXL, CRB)
        FBA = (KQG, JSC)
        KSF = (TLH, JBK)
        VFF = (CKT, GSP)
        PSG = (PQP, CBJ)
        LDZ = (JSC, KQG)
        DKN = (MVC, SPX)
        DRV = (DHD, GBJ)
        MJB = (HNM, LKH)
        MNG = (KXS, PRM)
        SNN = (VNQ, DQQ)
        GNH = (KHS, SLV)
        BJF = (HPD, XBL)
        LGP = (LHP, CKD)
        XVZ = (CVH, MRJ)
        GRP = (NHV, GBX)
        NGC = (FRJ, KVG)
        BJD = (QVR, RLF)
        FKL = (LRV, DSH)
        KSV = (VHK, VHK)
        XQK = (KPL, MDX)
        NRC = (DMT, GLC)
        VNH = (LQT, VVQ)
        CGL = (SBL, SBL)
        GVH = (NKD, XTK)
        DBJ = (LXQ, HKP)
        KVC = (PMX, XPR)
        NJP = (VXG, TTJ)
        VXG = (BQC, DNK)
        CLJ = (JVL, QRP)
        RFH = (VQL, NPJ)
        GFR = (KGB, GNM)
        KQV = (VFK, PKK)
        SQM = (GTJ, RKT)
        MQS = (MPN, HJC)
        BXV = (RJP, CLN)
        VSJ = (DKN, XDK)
        SHK = (XMV, RXM)
        GSP = (LDC, VPF)
        MSS = (VPS, HKF)
        SFM = (NVR, CQX)
        FMC = (BJF, SRS)
        DXS = (RMD, TLT)
        QHS = (CGM, GXT)
        DJP = (FDL, JHP)
        MPT = (PQF, DCB)
        LXQ = (PLS, PGK)
        RFD = (TTJ, VXG)
        JGJ = (HJS, TJT)
        XGN = (KVT, LQL)
        NBB = (LSX, XNR)
        CKD = (MMH, BPV)
        CTT = (CQX, NVR)
        CBJ = (RRG, KBD)
        CNG = (THV, QBK)
        BMD = (QPF, VHX)
        XLC = (BKL, SCZ)
        KQG = (CPT, VLL)
        KTJ = (KVC, VBM)
        JBG = (NJP, RFD)
        KVT = (NPT, GVS)
        QGP = (JVL, QRP)
        HFC = (THV, QBK)
        JDT = (HBS, RHL)
        MNF = (NRS, XVZ)
        GKR = (GTK, PCB)
        NVP = (NQQ, RRQ)
        MPQ = (GCS, KSF)
        PLS = (PRG, XQK)
        JDF = (NQQ, RRQ)
        CQD = (DBJ, VTV)
        LNP = (TPK, SCP)
        HQP = (XMV, RXM)
        RJM = (MJD, RPS)
        MQK = (FSL, CKH)
        KXS = (HBJ, HMP)
        CSX = (QCS, RKV)
        VCG = (LQL, KVT)
        NVJ = (JQG, GKD)
        NLB = (VLD, VFF)
        CQN = (NVP, JDF)
        PDL = (VTM, GBQ)
        SJM = (JNR, BGH)
        VTR = (HMK, RNS)
        QGS = (SQK, TPL)
        NFH = (XRH, DKD)
        TXQ = (GFL, PGH)
        GCJ = (XSL, BFB)
        RKT = (CQN, NNR)
        BFQ = (LBS, JLX)
        PKN = (FGD, TKG)
        XKP = (FDJ, BHH)
        QHX = (SPD, XCT)
        NKR = (GFR, KTV)
        GLQ = (FBM, KLD)
        PRR = (XSL, BFB)
        XLT = (HPB, MJJ)
        HRD = (QVP, VXH)
        SNL = (VSF, GTG)
        TBN = (XCS, LJX)
        RPS = (NFM, VNX)
        MKX = (XLG, SLT)
        NQC = (MPT, BLX)
        DKK = (FVQ, PFR)
        LCT = (TGK, PSH)
        XHV = (JSN, KFT)
        PRM = (HMP, HBJ)
        NQL = (BQD, TTN)
        HQV = (KVH, LBV)
        RKG = (HMK, RNS)
        LCL = (KQH, MKX)
        LMM = (PVJ, SXK)
        NCF = (XHV, FBS)
        NRS = (MRJ, CVH)
        RTK = (RJM, MJR)
        RFC = (NVJ, HNG)
        XXQ = (SXP, BJH)
        RPT = (NMV, VNV)
        QBR = (CTH, QKV)
        FXF = (VBM, KVC)
        CRB = (NQC, RXH)
        NQD = (BQN, RMJ)
        RKR = (KMQ, MGF)
        DSP = (GFL, PGH)
        DSH = (XTV, LVS)
        DMT = (QVN, PMS)
        VPS = (NNC, HKJ)
        JSC = (CPT, VLL)
        XKS = (DVT, DRH)
        TTV = (LKH, HNM)
        MJD = (VNX, NFM)
        SCZ = (SHQ, DLB)
        XVX = (CNG, HFC)
        KTV = (KGB, GNM)
        QHJ = (LSR, SSN)
        CQJ = (GPT, BMD)
        NMD = (TPK, SCP)
        DVB = (GSJ, GSJ)
        JMP = (MQP, RGQ)
        BSB = (BGC, KJC)
        CPT = (MRB, MHN)
        PQP = (RRG, KBD)
        RLG = (BJH, SXP)
        VQX = (XVR, JCB)
        XJN = (XRH, DKD)
        JHP = (QFT, MNG)
        KMB = (HRT, SJM)
        QSK = (PSG, CFJ)
        DKD = (VLF, JPT)
        NHV = (BGR, SDS)
        HXV = (XGQ, BSP)
        XXN = (DMT, GLC)
        VLM = (KMB, BTL)
        RLR = (DNR, LNB)
        PCH = (KQG, JSC)
        XRH = (VLF, JPT)
        SBL = (XLM, XLM)
        CRM = (JTJ, XKM)
        JSS = (BTS, LSV)
        CXM = (NMV, VNV)
        MLG = (HCT, HCT)
        JXH = (MVR, SDR)
        XBH = (XVF, STR)
        DXT = (VBN, JVK)
        JSN = (DTS, DJP)
        KLD = (MFK, PBD)
        KVH = (RFH, FPH)
        NJX = (GMV, VPQ)
        NRQ = (GKR, GMT)
        HXF = (DXS, QXR)
        DGS = (SNL, BVF)
        LJT = (RKR, QVK)
        JLX = (XGP, TKS)
        FCX = (PXP, GVH)
        VDX = (VBJ, TBR)
        VSF = (SPV, DDQ)
        VCX = (KTT, PPX)
        MMH = (MDM, FVS)
        MPB = (GBL, RRB)
        RLF = (TSM, DLN)
        RMJ = (BTQ, TBN)
        VJX = (QQP, HMV)
        XVF = (RFC, HXG)
        CTN = (VVV, TKM)
        QHD = (GBP, RLR)
        JGC = (JXH, QCQ)
        KPL = (FXF, KTJ)
        NGN = (MDH, VSJ)
        XPG = (MJB, TTV)
        MQP = (KTG, CQJ)
        JCB = (QHD, MVT)
        MNM = (RPT, CXM)
        RXH = (BLX, MPT)
        HHP = (GCJ, PRR)
        SLV = (JVD, TRF)
        HVT = (BJX, FND)
        MTD = (MMV, LNM)
        TTL = (QFH, LGP)
        LTV = (QCS, RKV)
        VXV = (TSX, HSM)
        TCM = (QHM, DHX)
        NBF = (LQT, VVQ)
        BTQ = (LJX, XCS)
        TKP = (KLD, FBM)
        LDC = (HSD, PJP)
        VHK = (NRS, NRS)
        NFM = (TMF, QFS)
        GTJ = (NNR, CQN)
        LSD = (QCK, BJD)
        HJV = (QBD, DGT)
        QBK = (MND, GJL)
        XNR = (MBF, SPN)
        HQB = (KXN, FJF)
        TNJ = (FDJ, BHH)
        TGK = (SQV, TLB)
        KMF = (VHK, MNF)
        JVK = (PDL, FFF)
        NSF = (VBJ, TBR)
        SCP = (QQR, PVV)
        XXP = (HHP, CDS)
        VHX = (NJX, KJF)
        VCJ = (JXH, QCQ)
        KJC = (KCS, SCR)
        XXT = (FJF, KXN)
        NNC = (VQD, NSR)
        VBM = (XPR, PMX)
        JTJ = (XDG, VHT)
        CNK = (CCX, MSS)
        FTF = (TTN, BQD)
        LKH = (SXJ, GMF)
        TLH = (CSR, VXV)
        GFF = (KML, QRX)
        LFR = (CLN, RJP)
        DTS = (FDL, JHP)
        TTN = (KTP, GFF)
        BTL = (HRT, SJM)
        HJS = (JCT, JMJ)
        HBJ = (JVR, GSF)
        SLF = (JDD, KJZ)
        CVH = (CNC, XLT)
        JGQ = (JGB, XKS)
        FGJ = (QFR, XVX)
        STQ = (XRQ, TCB)
        GCS = (JBK, TLH)
        FKK = (HVT, MPK)
        LJM = (DQM, XXG)
        BTS = (NMD, LNP)
        SVC = (NFP, QHX)
        VPF = (PJP, HSD)
        NPT = (NGC, FCK)
        TTK = (GBN, NRQ)
        XCT = (FNL, RSH)
        RRG = (TND, HCC)
        HMP = (GSF, JVR)
        TJV = (XXG, DQM)
        CTH = (CTT, SFM)
        TPK = (QQR, PVV)
        LSR = (PQH, LFT)
        XLM = (BKL, BKL)
        FGD = (VLM, LHT)
        GSF = (VTJ, HRD)
        PBN = (FGD, TKG)
        NMV = (NRC, XXN)
        CLN = (TPQ, CTX)
        TMF = (TSK, PTV)
        HPX = (FKC, PDT)
        GBN = (GMT, GKR)
        PCK = (PDX, HXV)
        TCB = (NCF, FJK)
        LJX = (NRH, PTN)
        QBD = (KVQ, GNH)
        CMC = (QBR, PBK)
        GBL = (HHR, GSC)
        SQN = (BJF, SRS)
        FCB = (QXR, DXS)
        GDJ = (XKS, JGB)
        PCB = (LCL, XHH)
        BPV = (MDM, FVS)
        PJD = (DVB, DVB)
        RHG = (MJR, RJM)
        DVT = (PKN, PBN)
        TFM = (QBR, PBK)
        HCC = (HQB, XXT)
        PGS = (DSX, MTV)
        TSR = (MQP, RGQ)
        KJF = (GMV, VPQ)
        GJS = (DKC, CKL)
        LNB = (QLK, DRT)
        PKK = (QPS, VJX)
        TKS = (RHG, RTK)
        QFS = (TSK, PTV)
        DGL = (RCK, XVJ)
        XPR = (MNM, GTR)
        TDH = (FKL, CFC)
        PDF = (XBP, CXB)
        DFN = (RXF, GRP)
        JVR = (HRD, VTJ)
        LBS = (TKS, XGP)
        QCQ = (MVR, SDR)
        MND = (JDT, QLP)
        PFR = (LJM, TJV)
        QLK = (MPB, PSV)
        CFC = (LRV, DSH)
        PMX = (GTR, MNM)
        CGF = (PMR, RPV)
        MVG = (LTH, KFN)
        NLK = (FBR, JPM)
        LFB = (SCS, PGS)
        BVF = (VSF, GTG)
        MDQ = (BVF, SNL)
        CQF = (PMR, RPV)
        FND = (LRX, PNM)
        MRJ = (CNC, XLT)
        HSD = (XBR, TCM)
        HLF = (CNP, GQH)
        RRQ = (CMC, TFM)
        CLM = (TKM, VVV)
        RLB = (TTV, MJB)
        NDS = (RQX, CQD)";

    let (instructions, node_map) = match data.split_once('\n') {
        Some(s) => s,
        None => panic!("Invalid input")
    };

    let instructions:Vec<char> = instructions.chars().collect();

    let node_map: HashMap<[char; 3], ([char; 3], [char; 3])> = parse_node_map(node_map);

    let starting_nodes: Vec<NodeDirections>  = node_map.iter()
        .filter(|(&k,_)| k[2] == 'A')
        .collect();

    let finish_nodes: Vec<NodeDirections>  = node_map.iter()
        .filter(|(&k,_)| k[2] == 'Z')
        .collect();

    let mut all_factors:Vec<(usize,usize)> = vec![];
    
    for starting_node in &starting_nodes {
        let mut steps = 0;
        let mut next_node = starting_node.0;
        for instruction in instructions.iter().cycle() {
            let direction = match node_map.get(next_node) {
                Some(n) => n,
                None => panic!()
            };

            next_node = match instruction {
                'L' => &direction.0,
                'R' => &direction.1,
                _ => panic!(),                
            };

            steps += 1;

            let a_finish:Vec<&NodeDirections> = finish_nodes.iter()
                .filter(|&(&s, _)| s == *next_node)
                .collect();

            if !a_finish.is_empty() {
                let seive = primal::Sieve::new(1000);
                let mut factors  = match seive.factor(steps) {

                    Ok(fac) => fac,
                    Err(_ ) => panic!()
                    
                };

                all_factors.append(factors.as_mut());
                break;
            }
        }
    }
    
    all_factors.sort();

    all_factors.dedup();

    let lcm:u64 = all_factors.iter()
        .map(|(f,p)| f.pow(*p as u32) as u64)
        .product();

    println!("lcm = {}",lcm);
}

fn parse_node_map(node_map: &str) -> HashMap<[char; 3], ([char; 3], [char; 3])> {
    let mut nodes:HashMap<[char;3],( [char;3], [char;3])> = HashMap::default();
    let mut window = [' ';16];
    
    for char in node_map.chars() {
        window.rotate_left(1);
        window[window.len() -1] = char;

        if let [n_1,n_2,n_3,' ','=',' ','(', l_1, l_2, l_3,',',' ', r_1, r_2, r_3,.. ] = window {
            let node = [n_1,n_2,n_3];
            let left = [l_1,l_2,l_3];
            let right = [r_1,r_2,r_3];
            if nodes.insert(node,(left,right)).is_some() { panic!("key already exists") };
        }
    }
    nodes
}
