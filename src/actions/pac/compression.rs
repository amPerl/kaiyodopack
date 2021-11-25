pub fn decompress(compressed: &[u8], decompressed: &mut [u8]) {
    let mut unk_array_256 = [0u8; 256];
    let mut unk_array_256_2 = [0u8; 256];

    let mut unk_struct_56_p_decompressed_idx = 0;
    let mut unk_struct_56_arr = vec![0u8; 52];

    let mut compressed_idx = 0;

    // eprintln!("compressed[0..16]:");
    // for val in &compressed[0..16] {
    //     eprint!("{:02X} ", val);
    // }
    // eprintln!();

    'top_level_loop: loop {
        let mut a = compressed[compressed_idx] as usize;
        let mut c = compressed_idx + 1;
        let mut b = 0_usize;

        // eprintln!("-- outer loop begin");
        // eprintln!("compressed_idx: {:X}", compressed_idx);
        // eprintln!("a: {:X}", a);

        // UnkDecompressed::Unk2
        {
            for (i, (val1, val2)) in unk_array_256
                .iter_mut()
                .zip(unk_array_256_2.iter_mut())
                .enumerate()
            {
                *val1 = i as u8;
                *val2 = 0;
            }
        }

        'second_loop: loop {
            // eprintln!("(second_loop) if a ({:X}) > 0x7F = {:?}", a, a > 0x7F);
            if a > 0x7F {
                b = b + a - 0x7F;
                a = 0;
            }

            // loc_408905
            if b >= 256 {
                break 'second_loop;
            }

            let mut v9 = a + 1;

            'second_inner_loop: loop {
                let v10 = compressed[c];
                c += 1;
                unk_array_256[b] = v10;

                if b != unk_array_256[b] as usize {
                    let v11 = compressed[c];
                    c += 1;
                    unk_array_256_2[b] = v11;
                }

                b += 1;
                v9 -= 1;

                if v9 == 0 {
                    break 'second_inner_loop;
                }
            }

            // loc_408935
            if b >= 256 {
                break;
            }

            a = compressed[c] as usize;
            c += 1;
        }

        // loc_408944
        let compressed_current_shifted = (compressed[c] as usize) << 8;
        let compressed_next_plain = compressed[c + 1] as usize;
        let mut v12 = compressed_next_plain + compressed_current_shifted;

        // eprintln!("v12: {0} {0:02X}h", v12);

        compressed_idx = c + 2;
        let mut d = v12;

        // eprintln!("compressed_idx: {0} {0:02X}h", compressed_idx);

        // 40895a
        let mut v13 = 0;

        'third_loop: loop {
            // eprintln!("v13: {0} {0:02X}h", v13);
            #[allow(unused_assignments)]
            let mut v14 = 0;

            if v13 == 0 {
                // loc_408969
                if v12 == 0 {
                    break 'third_loop;
                }

                d = v12 - 1;
                v14 = compressed[compressed_idx];
                compressed_idx += 1;
            } else {
                // loc_408960
                v13 -= 1;
                v14 = unk_struct_56_arr[v13];
            }

            // loc_408978
            let v15 = unk_array_256[v14 as usize];
            if v14 != v15 {
                // loc_40899b
                v13 += 2;
                unk_struct_56_arr[v13 - 2] = unk_array_256_2[v14 as usize];
                v12 = d;
                unk_struct_56_arr[v13 - 1] = v15;
            } else {
                // loc_40898a
                let v16 = unk_struct_56_p_decompressed_idx;
                decompressed[unk_struct_56_p_decompressed_idx] = v14;
                v12 = d;
                unk_struct_56_p_decompressed_idx = v16 + 1;
            }
        }

        if compressed_idx >= compressed.len() {
            break 'top_level_loop;
        }
    }
}
