use pf_sandbox::fighter::Action;
use enum_traits::ToIndex;

pub fn action_name_to_indexes(name: &str) -> Vec<usize> {
    match name {
        "ItemHandGrip" => vec!(),
        "ItemHandHave" => vec!(),
        "ItemHandPickUp" => vec!(),
        "ItemHandSmash" => vec!(),
        "Wait1" => vec!(Action::Idle),
        "Wait2" => vec!(),
        "Wait3" => vec!(),
        "WaitItem" => vec!(),
        "WalkBrake" => vec!(),
        "WalkFast" => vec!(),
        "WalkMiddle" => vec!(Action::Walk),
        "WalkSlow" => vec!(),
        "Dash" => vec!(Action::Dash),
        "Run" => vec!(Action::Run),
        "RunBrake" => vec!(Action::RunEnd),
        "Turn" => vec!(Action::TiltTurn),
        "TurnRun" => vec!(Action::RunTurn),
        "TurnRunBrake" => vec!(),
        "JumpAerialB" => vec!(Action::JumpAerialB),
        "JumpAerialF" => vec!(Action::JumpAerialF),
        "JumpB" => vec!(Action::JumpB),
        "JumpF" => vec!(Action::JumpF),
        "JumpSquat" => vec!(Action::JumpSquat),
        "DamageFall" => vec!(),
        "Fall" => vec!(Action::Fall),
        "FallAerial" => vec!(Action::AerialFall),
        "FallAerialB" => vec!(),
        "FallAerialF" => vec!(),
        "FallB" => vec!(),
        "FallF" => vec!(),
        "FallSpecial" => vec!(Action::SpecialFall),
        "FallSpecialB" => vec!(),
        "FallSpecialF" => vec!(),
        "LandingFallSpecial" => vec!(Action::SpecialLand),
        "LandingHeavy" => vec!(),
        "LandingLight" => vec!(Action::Land),
        "Squat" => vec!(Action::CrouchStart),
        "SquatRv" => vec!(Action::CrouchEnd),
        "SquatWait" => vec!(Action::Crouch),
        "SquatWaitItem" => vec!(),
        "StepAirPose" => vec!(),
        "StepBack" => vec!(),
        "StepFall" => vec!(),
        "StepJump" => vec!(),
        "StepPose" => vec!(),
        "Guard" => vec!(Action::Shield),
        "GuardOff" => vec!(Action::ShieldOff),
        "GuardOn" => vec!(Action::ShieldOn),
        "GuardDamage" => vec!(),
        "EscapeAir" => vec!(Action::AerialDodge),
        "EscapeB" => vec!(Action::RollB),
        "EscapeF" => vec!(Action::RollF),
        "EscapeN" => vec!(Action::SpotDodge),
        "Rebound" => vec!(),
        "Attack11" => vec!(),
        "Attack12" => vec!(),
        "Attack13" => vec!(),
        "AttackDash" => vec!(Action::DashAttack),
        "AttackS3Hi" => vec!(),
        "AttackS3Lw" => vec!(),
        "AttackS3S" => vec!(),
        "AttackHi3" => vec!(),
        "AttackLw3" => vec!(),
        "AttackS4Hold" => vec!(),
        "AttackS4S" => vec!(),
        "AttackS4Start" => vec!(),
        "AttackHi4" => vec!(),
        "AttackHi4Hold" => vec!(),
        "AttackHi4Start" => vec!(),
        "AttackLw4" => vec!(),
        "AttackLw4Hold" => vec!(),
        "AttackLw4Start" => vec!(),
        "AttackAirB" => vec!(Action::Bair),
        "AttackAirF" => vec!(Action::Fair),
        "AttackAirHi" => vec!(Action::Uair),
        "AttackAirLw" => vec!(Action::Dair),
        "AttackAirN" => vec!(Action::Nair),
        "LandingAirB" => vec!(Action::BairLand),
        "LandingAirF" => vec!(Action::FairLand),
        "LandingAirHi" => vec!(Action::UairLand),
        "LandingAirLw" => vec!(Action::DairLand),
        "LandingAirN" => vec!(Action::NairLand),
        "SpecialAirNEnd" => vec!(),
        "SpecialAirNLoop" => vec!(),
        "SpecialAirNStart" => vec!(),
        "SpecialNEnd" => vec!(),
        "SpecialNLoop" => vec!(),
        "SpecialNStart" => vec!(),
        "SpecialSAirAttack" => vec!(),
        "SpecialSAirDash" => vec!(),
        "SpecialSAirEnd" => vec!(),
        "SpecialSAirHold" => vec!(),
        "SpecialSAirStart" => vec!(),
        "SpecialSAttack" => vec!(),
        "SpecialSDash" => vec!(),
        "SpecialSEnd" => vec!(),
        "SpecialSHold" => vec!(),
        "SpecialSStart" => vec!(),
        "SpecialAirHi1" => vec!(),
        "SpecialAirHi2" => vec!(),
        "SpecialAirHi3" => vec!(),
        "SpecialAirHi4" => vec!(),
        "SpecialHi1" => vec!(),
        "SpecialHi2" => vec!(),
        "SpecialHi3" => vec!(),
        "SpecialHi4" => vec!(),
        "SpecialAirLw" => vec!(),
        "SpecialAirLwHit" => vec!(),
        "SpecialLw" => vec!(),
        "SpecialLwHit" => vec!(),
        "FinalAirStart" => vec!(),
        "FinalAirStartHit" => vec!(),
        "FinalAttack" => vec!(),
        "FinalEnd" => vec!(),
        "FinalFall" => vec!(),
        "FinalMove" => vec!(),
        "FinalStart" => vec!(),
        "FinalStartHit" => vec!(),
        "Catch" => vec!(),
        "CatchAttack" => vec!(),
        "CatchCut" => vec!(),
        "CatchDash" => vec!(),
        "CatchTurn" => vec!(),
        "CatchWait" => vec!(),
        "ThrowB" => vec!(),
        "ThrowF" => vec!(),
        "ThrowHi" => vec!(),
        "ThrowLw" => vec!(),
        "CaptureDamageHi" => vec!(),
        "CapturePulledHi" => vec!(),
        "CaptureWaitHi" => vec!(),
        "CaptureCut" => vec!(),
        "CaptureDamageLw" => vec!(),
        "CaptureJump" => vec!(),
        "CapturePulledLw" => vec!(),
        "CaptureWaitLw" => vec!(),
        "ThrownB" => vec!(),
        "ThrownDxB" => vec!(),
        "ThrownDxF" => vec!(),
        "ThrownDxHi" => vec!(),
        "ThrownDxLw" => vec!(),
        "ThrownF" => vec!(),
        "ThrownHi" => vec!(),
        "ThrownLw" => vec!(),
        "LightEat" => vec!(),
        "LightGet" => vec!(),
        "LightWalkEat" => vec!(),
        "LightWalkGet" => vec!(),
        "LightThrowB" => vec!(),
        "LightThrowDash" => vec!(),
        "LightThrowDrop" => vec!(),
        "LightThrowF" => vec!(),
        "LightThrowHi" => vec!(),
        "LightThrowLw" => vec!(),
        "LightThrowAirB" => vec!(),
        "LightThrowAirF" => vec!(),
        "LightThrowAirHi" => vec!(),
        "LightThrowAirLw" => vec!(),
        "HeavyGet" => vec!(),
        "HeavyThrowB" => vec!(),
        "HeavyThrowF" => vec!(),
        "HeavyThrowHi" => vec!(),
        "HeavyThrowLw" => vec!(),
        "HeavyWalk1" => vec!(),
        "HeavyWalk2" => vec!(),
        "SmashThrowAirB" => vec!(),
        "SmashThrowAirF" => vec!(),
        "SmashThrowAirHi" => vec!(),
        "SmashThrowAirLw" => vec!(),
        "SmashThrowB" => vec!(),
        "SmashThrowDash" => vec!(),
        "SmashThrowF" => vec!(),
        "SmashThrowHi" => vec!(),
        "SmashThrowLw" => vec!(),
        "Swing1" => vec!(),
        "Swing3" => vec!(),
        "Swing4" => vec!(),
        "Swing4Hold" => vec!(),
        "Swing4Start" => vec!(),
        "SwingDash" => vec!(),
        "ItemHammerAir" => vec!(),
        "ItemHammerMove" => vec!(),
        "ItemHammerWait" => vec!(),
        "Swing4Bat" => vec!(),
        "ItemScrew" => vec!(),
        "ItemScrewFall" => vec!(),
        "ItemDragoonGet" => vec!(),
        "ItemDragoonRide" => vec!(),
        "ItemBig" => vec!(),
        "ItemSmall" => vec!(),
        "ItemLegsBrakeB" => vec!(),
        "ItemLegsBrakeF" => vec!(),
        "ItemLegsDashB" => vec!(),
        "ItemLegsDashF" => vec!(),
        "ItemLegsFastB" => vec!(),
        "ItemLegsFastF" => vec!(),
        "ItemLegsJumpSquat" => vec!(),
        "ItemLegsLanding" => vec!(),
        "ItemLegsMiddleB" => vec!(),
        "ItemLegsMiddleF" => vec!(),
        "ItemLegsSlowB" => vec!(),
        "ItemLegsSlowF" => vec!(),
        "ItemLegsWait" => vec!(),
        "ItemShoot" => vec!(),
        "ItemShootAir" => vec!(),
        "ItemScopeAirEnd" => vec!(),
        "ItemScopeAirFire" => vec!(),
        "ItemScopeAirRapid" => vec!(),
        "ItemScopeAirStart" => vec!(),
        "ItemScopeEnd" => vec!(),
        "ItemScopeFire" => vec!(),
        "ItemScopeRapid" => vec!(),
        "ItemScopeStart" => vec!(),
        "ItemLauncher" => vec!(),
        "ItemLauncherAirFire" => vec!(),
        "ItemLauncherFall" => vec!(),
        "ItemLauncherFire" => vec!(),
        "ItemAssist" => vec!(),
        "GekikaraWait" => vec!(),
        "DamageHi1" => vec!(),
        "DamageHi2" => vec!(),
        "DamageHi3" => vec!(),
        "DamageLw1" => vec!(),
        "DamageLw2" => vec!(),
        "DamageLw3" => vec!(),
        "DamageN1" => vec!(Action::Damage),
        "DamageN2" => vec!(),
        "DamageN3" => vec!(),
        "DamageAir1" => vec!(Action::DamageFall),
        "DamageAir2" => vec!(),
        "DamageAir3" => vec!(),
        "DamageFlyHi" => vec!(),
        "DamageFlyLw" => vec!(),
        "DamageFlyN" => vec!(Action::DamageFly),
        "DamageFlyRoll" => vec!(),
        "DamageFlyTop" => vec!(),
        "DamageElec" => vec!(),
        "DownAttackU" => vec!(),
        "DownBackU" => vec!(),
        "DownBoundU" => vec!(Action::MissedTechIdle),
        "DownDamageU" => vec!(),
        "DownDamageU3" => vec!(),
        "DownEatU" => vec!(),
        "DownForwardU" => vec!(),
        "DownStandU" => vec!(),
        "DownWaitU" => vec!(),
        "DownAttackD" => vec!(),
        "DownBackD" => vec!(),
        "DownBoundD" => vec!(),
        "DownDamageD" => vec!(),
        "DownDamageD3" => vec!(),
        "DownEatD" => vec!(),
        "DownForwardD" => vec!(),
        "DownSpotD" => vec!(),
        "DownStandD" => vec!(),
        "DownWaitD" => vec!(),
        "Passive" => vec!(),
        "PassiveCeil" => vec!(),
        "PassiveStandB" => vec!(),
        "PassiveStandF" => vec!(),
        "PassiveWall" => vec!(),
        "PassiveWallJump" => vec!(),
        "FuraFura" => vec!(Action::Stun),
        "FuraFuraEnd" => vec!(),
        "FuraFuraStartD" => vec!(),
        "FuraFuraStartU" => vec!(),
        "FuraSleepEnd" => vec!(),
        "FuraSleepLoop" => vec!(),
        "FuraSleepStart" => vec!(),
        "Swallowed" => vec!(),
        "MissFoot" => vec!(),
        "Ottotto" => vec!(Action::Teeter),
        "OttottoWait" => vec!(Action::TeeterIdle),
        "Pass" => vec!(Action::PassPlatform),
        "StopCeil" => vec!(),
        "StopWall" => vec!(),
        "WallDamage" => vec!(),
        "CliffCatch" => vec!(),
        "CliffWait" => vec!(Action::LedgeIdle),
        "CliffAttackQuick" => vec!(Action::LedgeAttack),
        "CliffClimbQuick" => vec!(Action::LedgeGetup),
        "CliffEscapeQuick" => vec!(Action::LedgeRoll),
        "CliffJumpQuick1" => vec!(Action::LedgeJump),
        "CliffJumpQuick2" => vec!(),
        "CliffAttackSlow" => vec!(Action::LedgeAttackSlow),
        "CliffClimbSlow" => vec!(Action::LedgeGetupSlow),
        "CliffEscapeSlow" => vec!(Action::LedgeRollSlow),
        "CliffJumpSlow1" => vec!(Action::LedgeJumpSlow),
        "CliffJumpSlow2" => vec!(),
        "Slip" => vec!(),
        "SlipAttack" => vec!(),
        "SlipDash" => vec!(),
        "SlipDown" => vec!(),
        "SlipEscapeB" => vec!(),
        "SlipEscapeF" => vec!(),
        "SlipStand" => vec!(),
        "SlipTurn" => vec!(),
        "SlipWait" => vec!(),
        "Swim" => vec!(),
        "SwimEnd" => vec!(),
        "SwimF" => vec!(),
        "SwimRise" => vec!(),
        "SwimTurn" => vec!(),
        "SwimUp" => vec!(),
        "SwimUpDamage" => vec!(),
        "SwimDrown" => vec!(),
        "SwimDrownOut" => vec!(),
        "EntryL" => vec!(),
        "EntryR" => vec!(),
        "AppealHi" => vec!(Action::TauntUp),
        "AppealLw" => vec!(Action::TauntDown),
        "AppealS" => vec!(Action::TauntLeft, Action::TauntRight),
        "Lose" => vec!(),
        "Win1" => vec!(),
        "Win1Wait" => vec!(),
        "Win2" => vec!(),
        "Win2Wait" => vec!(),
        "Win3" => vec!(),
        "Win3Wait" => vec!(),
        "LadderCatchAirL" => vec!(),
        "LadderCatchAirR" => vec!(),
        "LadderCatchEndL" => vec!(),
        "LadderCatchEndR" => vec!(),
        "LadderCatchL" => vec!(),
        "LadderCatchR" => vec!(),
        "LadderDown" => vec!(),
        "LadderUp" => vec!(),
        "LadderWait" => vec!(),
        _ => {
            //println!("unmatched: {}", name); // TODO
            vec!()
        }
    }.iter().map(|x| x.index() as usize).collect()
}