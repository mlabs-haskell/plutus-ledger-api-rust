(function() {
    var implementors = Object.fromEntries([["plutus_ledger_api",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;<a class=\"struct\" href=\"https://docs.rs/num-bigint/0.4/num_bigint/bigint/struct.BigInt.html\" title=\"struct num_bigint::bigint::BigInt\">BigInt</a>&gt; for &amp;<a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"struct\" href=\"https://docs.rs/num-bigint/0.4/num_bigint/bigint/struct.BigInt.html\" title=\"struct num_bigint::bigint::BigInt\">BigInt</a>&gt; for <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>&gt; for <a class=\"struct\" href=\"https://docs.rs/num-bigint/0.4/num_bigint/bigint/struct.BigInt.html\" title=\"struct num_bigint::bigint::BigInt\">BigInt</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;'a <a class=\"struct\" href=\"https://docs.rs/num-bigint/0.4/num_bigint/bigint/struct.BigInt.html\" title=\"struct num_bigint::bigint::BigInt\">BigInt</a>&gt; for <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i16.html\">i16</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i32.html\">i32</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i64.html\">i64</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i8.html\">i8</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u16.html\">u16</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u32.html\">u32</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u64.html\">u64</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u8.html\">u8</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>&gt; for <a class=\"struct\" href=\"https://docs.rs/num-bigint/0.4/num_bigint/bigint/struct.BigInt.html\" title=\"struct num_bigint::bigint::BigInt\">BigInt</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i16.html\">i16</a>&gt; for &amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i32.html\">i32</a>&gt; for &amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i64.html\">i64</a>&gt; for &amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.i8.html\">i8</a>&gt; for &amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u16.html\">u16</a>&gt; for &amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u32.html\">u32</a>&gt; for &amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u64.html\">u64</a>&gt; for &amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u8.html\">u8</a>&gt; for &amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"struct\" href=\"https://docs.rs/num-bigint/0.4/num_bigint/bigint/struct.BigInt.html\" title=\"struct num_bigint::bigint::BigInt\">BigInt</a>&gt; for &amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;<a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>&gt; for &amp;'a <a class=\"struct\" href=\"https://docs.rs/num-bigint/0.4/num_bigint/bigint/struct.BigInt.html\" title=\"struct num_bigint::bigint::BigInt\">BigInt</a>"],["impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/arith/trait.Mul.html\" title=\"trait core::ops::arith::Mul\">Mul</a>&lt;&amp;'a <a class=\"struct\" href=\"plutus_ledger_api/v1/value/struct.Value.html\" title=\"struct plutus_ledger_api::v1::value::Value\">Value</a>&gt; for &amp;'b <a class=\"struct\" href=\"https://docs.rs/num-bigint/0.4/num_bigint/bigint/struct.BigInt.html\" title=\"struct num_bigint::bigint::BigInt\">BigInt</a>"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[10223]}