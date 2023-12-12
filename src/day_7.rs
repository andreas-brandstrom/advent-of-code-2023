use std::{char, collections::HashMap};

#[derive(Eq, Debug)]
struct Game([u32; 5],u32, Type);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPairs,
    TreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Game {
    fn from(hand:[char; 5],bid: u32) -> Self {
        let mut number_of_kinds = (0,0,0,0);


        let mut numbers_of_j = 0;
        let numeric_value_hand:Vec<u32> = hand.iter().map(|&c| {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'T' => 10,
                '2'..='9' => c.to_digit(10).unwrap(),
                'J' => {numbers_of_j += 1; 1},
                _ => panic!()
            }
        }).collect();
        
        let numeric_value_hand: [u32; 5] = [
            numeric_value_hand[0],
            numeric_value_hand[1],
            numeric_value_hand[2],
            numeric_value_hand[3],
            numeric_value_hand[4],
        ];

        let mut cards: HashMap<u32, u32> = HashMap::default();
        for card in numeric_value_hand {
            match cards.get_mut(&card) {
                Some(c ) => *c += 1,
                None => {cards.insert(card, 1);},
            };
        }

        for nr_cards in cards {
            match nr_cards.1 {
                5 => number_of_kinds.0 += 1,
                4 => number_of_kinds.1 += 1,
                3 => number_of_kinds.2 += 1,
                2 => number_of_kinds.3 += 1,
                _ => ()
            }
        }

        let card_type = match number_of_kinds {
            (1,..) => Type::FiveKind,
            (0, 1, ..) => Type::FourKind,
            (0,0,1,1,..) => Type::FullHouse,
            (0,0,1,..) => Type::TreeKind,
            (0,0,0,2,..) => Type::TwoPairs,
            (0,0,0,1,..) => Type::OnePair,
            _ => Type::HighCard  
        };

        let card_type = match (card_type, numbers_of_j) {
            (Type::FourKind,1 | 4) => Type::FiveKind,
            (Type::FullHouse,2 | 3) => Type::FiveKind,
            (Type::TreeKind,1 | 3) => Type::FourKind,
            (Type::TwoPairs,2) => Type::FourKind,
            (Type::TwoPairs,1) => Type::FullHouse,
            (Type::OnePair,1..=2) => Type::TreeKind,
            (Type::HighCard,1) => Type::OnePair,
            (c_t, ..) => c_t,
        };

        Game(numeric_value_hand, bid, card_type)
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.2.cmp(&other.2) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        };

        let mut self_iter = self.0.iter();
        let mut other_iter = other.0.iter();

        while let (Some(&c_self),Some( &c_other)) = (self_iter.next(), other_iter.next()) {
            match c_self.cmp(&c_other) {
                std::cmp::Ordering::Equal => {}
                ord => return ord
            }
        };

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}



pub(crate) fn camel_cards() {
    let data = 
        "TQA26 14
        66656 612
        JK3JT 442
        A6QQA 214
        Q3777 131
        J2K8J 180
        J56KK 578
        33699 532
        999K4 605
        4J445 884
        59A5A 381
        86686 419
        7T5KJ 92
        47883 178
        749K3 2
        7K6J3 675
        8J22J 869
        47463 168
        2A5K3 597
        27332 465
        4AQ9K 236
        59TJ5 883
        5TTTQ 333
        A8QAJ 208
        JAJ95 380
        8688A 265
        34K62 525
        KT7J7 709
        A5A5A 890
        7347J 935
        6TQ57 973
        888K8 345
        J5456 33
        8T8T4 456
        3J942 88
        QJJJA 727
        75333 479
        K6622 369
        T468J 608
        835A7 107
        67AA7 390
        QTJ9Q 25
        Q56Q3 579
        7K245 227
        36857 327
        8J7K9 524
        7AAAJ 277
        JKK4K 352
        43974 576
        2QJ22 482
        99JJ9 484
        AJAQQ 341
        8222K 976
        JA57Q 84
        A593T 725
        9T494 581
        TKTTT 941
        KJ55A 439
        9444J 137
        4423T 504
        K9J96 944
        6K9JQ 656
        2Q34J 48
        8T87Q 679
        5J955 27
        Q3T4T 416
        J8888 972
        5Q298 257
        64463 937
        44K4J 708
        KQQQQ 622
        TJKTT 800
        9394J 63
        TJ639 660
        K4JAQ 119
        9838Q 788
        76K4A 38
        J86KA 768
        3AQ3A 617
        J2592 40
        QAK68 761
        98999 620
        J4T34 963
        2A979 483
        95JA8 336
        A33AA 54
        TTT33 518
        QT759 212
        A7A3A 999
        KJ6AK 485
        99339 921
        TJ3A4 968
        2TKTT 573
        28626 844
        9KK8K 79
        7QT7A 57
        66777 493
        JJ355 676
        AJAAA 176
        AAQA3 522
        T4KJ4 308
        375K3 658
        J7K75 864
        JA275 669
        76T45 507
        AJT92 975
        K5434 346
        942Q2 598
        JA33A 495
        3JKK4 39
        2AAJA 399
        6555T 296
        7962T 787
        KJK2K 764
        684JJ 998
        6JJ36 52
        64K66 838
        JA8K3 901
        3Q7Q3 774
        9JK99 666
        3TK6J 355
        KKJ25 503
        86658 584
        T4T42 268
        Q4238 530
        T632K 700
        TJTTJ 571
        T7Q38 747
        AJ6KQ 150
        2QT22 885
        KJA4A 784
        87667 859
        63994 570
        Q82A6 331
        TK9T6 124
        J5K5K 140
        TKAJA 211
        59AAA 683
        89J57 694
        TQTT9 122
        59996 715
        73TQA 397
        AJT3T 420
        85882 777
        AA4AT 175
        4K4K7 670
        8JA79 19
        QKKJ8 340
        9TTTA 243
        77QQ7 314
        22237 22
        656QK 911
        8K79A 847
        4K466 734
        77979 924
        4J848 949
        J72K4 388
        77JJ7 741
        77477 724
        8558K 444
        668K9 441
        448T4 820
        AQA7A 629
        Q93J3 732
        4344J 582
        JA2JA 623
        44422 735
        3444Q 652
        5TQQQ 929
        A5T6J 216
        855JK 808
        Q7QKK 887
        3J433 297
        QJQ77 742
        6JJ99 492
        2QA2J 717
        326AA 704
        JJJJJ 194
        3QQQJ 550
        4J424 445
        64A53 201
        9J5AA 718
        A8747 438
        33788 989
        3K399 255
        T4343 400
        444J4 382
        3J3J3 12
        J7552 228
        352Q8 812
        8A833 762
        89638 940
        42AQK 577
        6TT68 326
        AAQAA 102
        4A37A 647
        TAJQ7 991
        85585 395
        66366 406
        9Q9AQ 177
        K6K6K 785
        2T228 143
        K9Q4Q 109
        T7999 462
        28294 978
        2T3J7 515
        A2AJ6 886
        AATAA 508
        KJ73J 86
        7AQ26 376
        6K97T 163
        A4AA6 537
        JK555 726
        2TTK5 356
        98AQ8 601
        2JJ4K 771
        K2386 791
        55888 254
        555JJ 690
        799KK 378
        44A62 786
        4338K 246
        338J3 688
        Q39J2 139
        22J52 338
        6KAJ5 933
        95J98 332
        67K4J 317
        4K545 592
        9Q929 852
        68872 157
        59535 548
        AAKJK 615
        36K97 758
        TQQTT 394
        QQ8QQ 926
        Q5K9Q 75
        JJAQ6 637
        JTJ32 275
        22TQA 729
        KQK3K 203
        43J86 104
        A6663 11
        9QJT6 908
        AT3A6 430
        56245 827
        6Q334 604
        77T7T 841
        TQK68 668
        22A2A 773
        62TK6 892
        8K898 912
        Q3Q26 411
        59678 26
        43924 451
        AT3K3 952
        78788 16
        24229 71
        J947T 849
        5K5Q2 897
        92332 42
        TT3T4 32
        47995 686
        99T99 749
        K8QQQ 408
        82889 846
        A6A22 93
        4A76J 590
        62222 795
        TTTT8 316
        7QJ68 696
        K9J9J 913
        944JJ 135
        77757 722
        65J66 707
        J7838 4
        2T895 802
        2AAAA 325
        Q855Q 619
        95223 148
        87989 706
        K4A79 96
        22T86 60
        A5AAA 43
        Q6666 339
        54445 320
        88JJ8 262
        36AA9 461
        6J6JQ 121
        8K7T6 114
        9A863 377
        T7TA8 126
        JJJTT 415
        64JTK 289
        K39K3 757
        54K89 737
        73377 372
        T5389 365
        4949K 304
        87787 763
        KQ33K 367
        Q75TK 721
        999QQ 934
        46647 433
        88886 803
        7J2AK 888
        KKTKT 10
        222T2 440
        JA78A 239
        J6KKK 610
        AAA8A 166
        65J62 545
        57355 455
        5AQ43 240
        KKTKJ 59
        3J337 170
        J5AK4 872
        TTQQJ 811
        TTT7T 627
        Q8877 588
        T5368 337
        A4K3T 662
        9AAAT 136
        TJ5K5 402
        332A8 89
        T6T6T 280
        T6985 603
        966Q9 681
        88444 458
        27JJ8 473
        66669 5
        4JJ48 881
        44426 335
        88A8A 85
        A5A36 323
        7QQ57 984
        464T6 745
        636QA 155
        88TAQ 879
        62874 421
        535T5 50
        4Q444 470
        83432 501
        24444 586
        47774 539
        KJKJ9 53
        A4355 920
        QTKKK 290
        333A3 540
        TTTAT 965
        28622 611
        35QA9 387
        94J48 797
        699T8 123
        66266 422
        QQ44Q 919
        8J9KK 974
        Q9Q76 685
        66J66 945
        7ATTJ 334
        66KTK 904
        TT484 932
        52557 792
        2828J 995
        JJ9JK 635
        9745J 55
        22299 549
        92QJK 222
        QAA7J 531
        7A5J8 358
        22KK2 544
        27Q23 273
        AJ88A 602
        33A6A 459
        22T26 232
        4AATT 174
        T5342 743
        55255 979
        Q88QQ 284
        222A2 680
        3QQ33 554
        K2KQ2 950
        AAA6J 650
        5Q333 62
        7J574 37
        38KK8 360
        K2K2K 760
        4T9TK 463
        6Q67J 472
        66686 778
        44J4J 990
        Q3993 655
        77J27 389
        TTQ27 35
        277K8 446
        27J42 428
        KK4J4 798
        QTK6J 199
        4344A 457
        TQTKQ 977
        8833J 237
        99TJK 147
        6Q9Q4 313
        TTJKK 98
        33833 460
        83T3A 805
        TJJ22 754
        5TAJK 632
        T293Q 191
        55757 70
        2TT8T 851
        7Q575 61
        22K4K 828
        8T288 204
        A8AA8 928
        KJ68J 697
        45AAJ 213
        8QJ75 693
        23KKK 319
        Q2Q22 969
        224J4 821
        44Q33 583
        34KJ3 342
        4JA57 687
        88T2Q 354
        989A6 51
        AAQA9 631
        5A57J 311
        62266 77
        2A84A 132
        99JKK 234
        88899 776
        K3395 6
        968JA 830
        6J444 282
        QA55Q 251
        39K99 209
        4A69Q 790
        77K57 562
        T5QT5 115
        56455 134
        T3TQQ 283
        46A6A 413
        62T67 626
        5757A 371
        98A47 642
        2T653 813
        2888A 322
        TTQ8T 454
        93333 876
        28AAA 580
        QA63Q 196
        2Q5QK 184
        48TTJ 300
        9JT79 306
        77T77 930
        6K666 957
        6J696 417
        8J588 703
        666TJ 673
        QKKJ7 233
        577AA 286
        8J775 781
        939KK 866
        34444 558
        4A4AA 221
        523QT 630
        8K259 651
        2T34K 716
        6K2T5 8
        88T83 551
        Q34AK 142
        4444T 7
        JTKT2 621
        JQJJQ 288
        38344 599
        99292 74
        Q36AA 206
        52255 871
        33J92 188
        K27QA 318
        66268 513
        29QA6 613
        74J74 815
        5KK6K 895
        Q9J9Q 511
        6J33J 158
        QAQQA 903
        KTQQQ 305
        2T759 958
        A796K 108
        63266 568
        KTT7K 691
        Q5A5A 186
        47787 267
        6T6TQ 58
        56472 891
        K7953 648
        99A9A 486
        Q78K2 249
        99799 705
        2K62A 671
        6TTJT 392
        4T44T 878
        44KKK 410
        KTAQ7 423
        222Q2 244
        4J324 401
        63636 575
        9K54J 856
        K7K77 755
        2K222 160
        3J644 711
        3355J 259
        A9995 664
        43586 909
        A4744 574
        Q7337 309
        7QQ27 161
        J8575 374
        9K99K 636
        7TJT4 674
        79888 505
        K3AJ5 730
        K4QJK 427
        99TJ9 263
        6963T 905
        3K3J5 474
        TJ4J8 806
        5TA38 437
        866J6 807
        2T454 44
        63A4J 865
        879T8 527
        A5282 217
        53737 951
        44Q9Q 634
        TTJT9 210
        QK67K 192
        33A3A 915
        88Q8J 125
        QJ2QQ 256
        2888J 20
        2786T 391
        J27J7 938
        2AAT5 521
        544Q4 528
        A4645 30
        AKAA2 526
        44535 835
        KQ33Q 585
        66JJ6 818
        9979J 138
        TKKKK 661
        KA225 278
        Q8888 794
        K27QK 916
        83323 494
        39969 684
        53359 986
        77J73 299
        5TK9J 782
        222JJ 713
        3Q383 753
        46466 298
        QQQJJ 875
        Q2394 860
        QA5QA 988
        8K5K9 877
        7Q78Q 68
        AA6A6 923
        J565K 29
        52766 301
        8598Q 159
        KKK8K 824
        8KTQ5 49
        66486 41
        45459 842
        67676 223
        44283 971
        22T5T 836
        88857 164
        8K492 250
        939J9 900
        K5J9K 779
        5Q692 145
        8T88T 907
        55535 31
        8K822 609
        9TQK9 67
        6762J 435
        T5T5T 780
        4K442 152
        657J6 202
        44494 349
        78377 925
        32858 882
        276Q5 359
        49526 509
        2J623 918
        4222A 914
        TTTTJ 939
        255TQ 179
        KJ42T 898
        79268 404
        648K7 189
        6T7J7 488
        37Q63 538
        K366J 469
        KKK67 156
        998K6 242
        QQQ7Q 560
        33223 231
        9A999 167
        28228 961
        JJK2K 141
        KKK46 633
        KT3KA 845
        A2Q2A 667
        AA9AA 471
        A663A 76
        J966Q 996
        77J76 127
        74JJ7 748
        6QQQ7 303
        8AJ44 18
        J2J34 480
        A289Q 195
        8335Q 266
        QQ2Q2 385
        45K4J 870
        48JK6 1000
        T9949 294
        2585K 272
        7J77K 809
        J53KK 657
        AAAT5 347
        72J8K 476
        TQ77T 910
        29Q7T 767
        4TTTT 663
        642AA 292
        37QQQ 171
        86Q74 97
        96J88 34
        QQ7QJ 618
        77772 546
        99559 23
        AJ666 295
        AAKAK 964
        K8Q32 120
        8QAK5 563
        A9K9K 45
        3475K 362
        95999 739
        482TQ 36
        5Q555 452
        854Q3 894
        QTQQQ 561
        55755 99
        899AJ 235
        QAQ22 502
        75353 638
        K6A87 572
        J2KT7 366
        KA39K 144
        8AAAJ 409
        66753 766
        TT626 510
        AJ999 418
        42222 983
        5J577 994
        262KK 330
        99992 701
        4K334 825
        96J96 536
        774K7 229
        A9362 351
        KA8KJ 982
        J2552 594
        TTT2T 810
        KK24K 640
        23222 162
        K36Q7 321
        QQJTJ 519
        33384 606
        5A834 500
        Q35Q2 379
        A7888 434
        595J2 47
        2K7AA 728
        AA66J 9
        AKKKA 373
        53335 328
        447J4 514
        KT7K7 21
        785TT 253
        3J783 720
        7J2AQ 405
        56Q5Q 447
        Q647K 94
        28488 207
        694T2 714
        KJKKK 78
        8T866 130
        274Q5 672
        89992 312
        J2242 350
        QQ988 899
        3T3T3 464
        Q2226 752
        55222 639
        7T9T9 271
        Q8884 832
        2Q92Q 516
        46886 817
        77J7T 829
        AAA9J 80
        35J97 555
        358K7 692
        ATT6T 678
        K2465 698
        A2A59 46
        58T24 956
        8ATA2 804
        AAK3K 751
        28897 649
        QAQKQ 854
        7T776 353
        26226 426
        5566T 13
        8KKTK 857
        TT44J 512
        QA29K 172
        A3AAA 28
        6TTQT 993
        8TTJT 922
        44TT5 942
        K67QT 699
        AA77J 927
        J8JJJ 153
        4T474 73
        4QJ8T 475
        86386 796
        AA2QK 218
        86A4K 205
        78J84 481
        27K3T 260
        AAKTQ 641
        6QQ33 467
        3K333 279
        62888 889
        8Q388 185
        Q88QK 490
        9494J 24
        5T888 783
        36633 769
        8T484 258
        82878 987
        J3336 643
        TKKAK 276
        822AJ 429
        72727 917
        254JK 113
        58888 843
        AK537 624
        QQQJ8 659
        J3366 274
        ATTJK 468
        5KKK2 72
        AAAKA 533
        5525J 962
        Q6QJ6 165
        5K424 607
        94KK9 66
        43544 487
        JJ38T 712
        6K6KJ 90
        J7788 936
        75522 625
        K6KK9 955
        AJ7J7 765
        K7QTQ 407
        37333 837
        82J83 523
        2A526 543
        99TJJ 238
        33343 997
        44446 361
        8868K 432
        9J989 82
        49999 65
        5TQ78 966
        AAA48 534
        76J5J 946
        J9J25 187
        5T973 744
        KK562 816
        486A4 324
        569TQ 269
        3AAT4 343
        TQTTT 386
        T56A9 449
        Q844Q 535
        T8T25 281
        QJ5J5 858
        AKTK8 364
        33532 315
        92959 943
        999A2 198
        TAQ6J 770
        29295 616
        AKA7Q 833
        55A5A 896
        KK49A 396
        ATJ84 197
        73K87 383
        6A56A 478
        7KQT3 775
        49A93 110
        2928A 736
        5JK35 95
        53JQA 261
        TT399 443
        57779 287
        77296 220
        3T334 436
        54K87 756
        45455 393
        647J4 868
        5555J 498
        K877K 173
        646JJ 589
        2JJAJ 960
        7T5T6 293
        T3K3T 118
        TJA63 855
        7K7AK 403
        68754 614
        J5628 822
        J93TT 245
        JQJQ9 874
        32JT2 823
        7AA77 948
        AQTQA 56
        48986 252
        9K382 116
        TJ9T9 230
        Q7632 193
        998K9 750
        KK99K 491
        Q57J2 219
        T58T9 596
        66T6T 424
        QK354 953
        66652 863
        23J33 151
        7J337 653
        3J247 215
        A7KJ5 302
        A555K 654
        KQ9TT 363
        99Q49 370
        2JJ65 587
        7TAAJ 553
        KKK68 826
        77755 850
        J74T2 959
        2TQ2Q 954
        ATJ3Q 344
        45448 557
        5JJ5J 738
        T3278 595
        465AQ 520
        97Q82 665
        99779 183
        KQJAA 91
        KQTJT 746
        Q2542 453
        99J99 414
        39T99 368
        AA4AJ 906
        8TK28 529
        5A55J 819
        TT4T4 853
        JQAQQ 100
        57T4A 970
        AJAA3 834
        9JTT7 291
        9AK6A 902
        96J94 181
        3J944 593
        94KQ4 375
        KKKK2 556
        88J99 567
        JAAAJ 840
        9J9Q9 128
        74447 81
        28288 719
        32K88 225
        53535 87
        74555 224
        6972Q 628
        7JQJ8 893
        2T25J 190
        92226 552
        Q3QTQ 591
        A666A 814
        26247 103
        4KTKK 981
        J22K2 133
        3KKKK 873
        KJK7K 689
        6A6K2 985
        3AAAT 425
        22J27 839
        8888T 677
        6A666 569
        8887Q 497
        696Q3 799
        99646 695
        KKK5K 69
        3A332 106
        Q7TTT 831
        96622 566
        4JAA6 564
        22572 600
        92AKJ 931
        TA8AA 848
        73373 112
        6666T 182
        AJKJ7 645
        7A7AA 496
        T5555 740
        J22JJ 967
        24AAA 146
        956AQ 412
        66A9J 329
        46644 477
        A9939 105
        Q5Q5Q 702
        588Q5 384
        63AAJ 466
        254KA 947
        8KK88 565
        KQ29T 541
        36K66 759
        7JAQQ 431
        8QTT4 992
        3333J 731
        44447 793
        T355T 489
        39723 862
        828QQ 270
        96999 15
        333Q2 733
        T774T 559
        55K55 1
        9J955 264
        K2KQK 682
        QKQKQ 801
        222J2 247
        97J79 3
        42TT3 499
        2KQ69 129
        4764T 450
        72K2K 83
        3Q334 644
        TTT88 348
        2Q223 310
        J43A3 880
        J4229 64
        4QJ8A 17
        43434 285
        92222 772
        T59K6 448
        66674 248
        34334 241
        JK529 547
        T7TT3 517
        22QJQ 117
        96J6J 861
        QTJT4 101
        Q444J 307
        T2Q8K 357
        93T24 789
        T7K57 398
        2K94A 111
        9J942 200
        T7J8K 646
        55665 980
        56555 169
        79777 149
        82A93 710
        Q989Q 226
        J2T55 542
        763K2 723
        QQQJQ 867
        QQQQA 506
        J7777 154";

    let mut games = parse_games(data);

    games.sort();

    let mut sum = 0;

    for (i, game) in games.iter().enumerate() {
        let i = i as u32;
        sum += (i+1_u32)*game.1;
    }

    println!("Sum {}",sum);
}

fn parse_games(data: &str) -> Vec<Game> {
    let mut data_iter = data.split_whitespace();

    let mut games: Vec<Game> = vec![];

    while let (Some(hand), Some(bid)) = (data_iter.next(), data_iter.next()) {
        let bid   = bid.parse().unwrap();
        let hand:Vec<char> = hand.chars().collect();
        match hand[..] {
            [a,b,c,d,e] => games.push(Game::from([a,b,c,d,e], bid)) ,
            _ => panic!("Invalid input")
        }
    }

    games
}
