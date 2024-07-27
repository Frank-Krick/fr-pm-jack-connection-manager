use crate::domain::audio_interface::AudioInterface;
use crate::domain::audio_interface::umc_1820::Umc1820;
use crate::domain::digital_drum_section::DigitalDrumSection;
use crate::domain::group_mixer_section::GroupMixerSection;
use crate::domain::jack_connection_manager::JackConnectionManager;
use crate::domain::jack_mixer::JackMixer;
use crate::domain::main_input_mixer_section::MainInputMixerChannel;
use crate::domain::zyn_add_sub_fx::ZynAddSubFx;

mod domain;

fn main() {
    let jack_connection_manager = JackConnectionManager::new();
    let digital_drum_section = DigitalDrumSection::new();

    digital_drum_section.connect_internals(&jack_connection_manager);

    let main_input_mixer_section = MainInputMixerChannel::new();

    main_input_mixer_section.connect_internals(&jack_connection_manager);

    connect_digital_drum_section(
        &jack_connection_manager,
        &digital_drum_section,
        &main_input_mixer_section);

    let audio_interface = Umc1820::new();

    connect_audio_interface(
        &jack_connection_manager,
        &main_input_mixer_section,
        &audio_interface);

    let zyn_add_sub_fx = ZynAddSubFx::new();

    connect_zyn_sub_add_fx(
        &jack_connection_manager,
        &main_input_mixer_section,
        &zyn_add_sub_fx);

    connect_send_to_torso_s4(
        &jack_connection_manager,
        &main_input_mixer_section,
        &audio_interface);

    let group_mixer_section = GroupMixerSection::new();
    group_mixer_section.connect_internals(&jack_connection_manager);

    connect_group_section_to_audio_output(
        &jack_connection_manager,
        &audio_interface,
        &group_mixer_section);

    connect_drum_channels(
        &jack_connection_manager,
        &main_input_mixer_section,
        &group_mixer_section);
}

fn connect_drum_channels(jack_connection_manager: &JackConnectionManager,
                         main_input_mixer_section: &MainInputMixerChannel,
                         group_mixer_section: &GroupMixerSection) {
    jack_connection_manager.connect(
        main_input_mixer_section.mixer().output_channels_by_name("DSMPL")[0],
        &group_mixer_section.channel_inputs(0)[0]
    );
    jack_connection_manager.connect(
        main_input_mixer_section.mixer().output_channels_by_name("DSMPL")[1],
        &group_mixer_section.channel_inputs(0)[1]
    );

    jack_connection_manager.connect(
        main_input_mixer_section.mixer().output_channels_by_name("DFire")[0],
        &group_mixer_section.channel_inputs(0)[0]
    );
    jack_connection_manager.connect(
        main_input_mixer_section.mixer().output_channels_by_name("DFire")[1],
        &group_mixer_section.channel_inputs(0)[1]
    );

    jack_connection_manager.connect(
        main_input_mixer_section.mixer().output_channels_by_name("DEuro")[0],
        &group_mixer_section.channel_inputs(0)[0]
    );
    jack_connection_manager.connect(
        main_input_mixer_section.mixer().output_channels_by_name("DEuro")[1],
        &group_mixer_section.channel_inputs(0)[1]
    );
}

fn connect_group_section_to_audio_output(jack_connection_manager: &JackConnectionManager,
                                         audio_interface: &Umc1820,
                                         group_mixer_section: &GroupMixerSection) {
    jack_connection_manager.connect(
        &group_mixer_section.output_channels()[0],
        &audio_interface.input_channels()[0]
    );
    jack_connection_manager.connect(
        &group_mixer_section.output_channels()[1],
        &audio_interface.input_channels()[1]
    );
}

fn connect_send_to_torso_s4(jack_connection_manager: &JackConnectionManager,
                            main_input_mixer_section: &MainInputMixerChannel,
                            audio_interface: &Umc1820) {
    let send_mixer_t = main_input_mixer_section.send_mixer(4);
    jack_connection_manager.connect(
        &send_mixer_t.output_channels()[0],
        &audio_interface.input_channels()[2]
    );
    jack_connection_manager.connect(
        &send_mixer_t.output_channels()[1],
        &audio_interface.input_channels()[3]
    );
}

fn connect_zyn_sub_add_fx(jack_connection_manager: &JackConnectionManager,
                          main_input_mixer_section: &MainInputMixerChannel,
                          zyn_add_sub_fx: &ZynAddSubFx) {
    jack_connection_manager.connect(
        &zyn_add_sub_fx.output_channels()[0],
        main_input_mixer_section.channel_inputs(7)[0]
    );
    jack_connection_manager.connect(
        &zyn_add_sub_fx.output_channels()[1],
        main_input_mixer_section.channel_inputs(7)[1]
    );
    jack_connection_manager.connect(
        &zyn_add_sub_fx.output_channels()[0],
        main_input_mixer_section.channel_inputs(7)[2]
    );
    jack_connection_manager.connect(
        &zyn_add_sub_fx.output_channels()[1],
        main_input_mixer_section.channel_inputs(7)[3]
    );
}

fn connect_audio_interface(jack_connection_manager: &JackConnectionManager,
                           main_input_mixer_section: &MainInputMixerChannel,
                           audio_interface: &Umc1820) {
    for channel_number in 0..6 {
        jack_connection_manager.connect(
            &audio_interface.output_channels()[2 * channel_number],
            main_input_mixer_section.channel_inputs(channel_number + 1)[0]
        );
        jack_connection_manager.connect(
            &audio_interface.output_channels()[2 * channel_number + 1],
            main_input_mixer_section.channel_inputs(channel_number + 1)[1]
        );
        jack_connection_manager.connect(
            &audio_interface.output_channels()[2 * channel_number],
            main_input_mixer_section.channel_inputs(channel_number + 1)[2]
        );
        jack_connection_manager.connect(
            &audio_interface.output_channels()[2 * channel_number + 1],
            main_input_mixer_section.channel_inputs(channel_number + 1)[3]
        );
    }

    jack_connection_manager.connect(
        &audio_interface.output_channels()[2 * 6],
        main_input_mixer_section.channel_inputs(8)[0]
    );
    jack_connection_manager.connect(
        &audio_interface.output_channels()[2 * 6 + 1],
        main_input_mixer_section.channel_inputs(8)[1]
    );
    jack_connection_manager.connect(
        &audio_interface.output_channels()[2 * 6],
        main_input_mixer_section.channel_inputs(8)[2]
    );
    jack_connection_manager.connect(
        &audio_interface.output_channels()[2 * 6 + 1],
        main_input_mixer_section.channel_inputs(8)[3]
    );

    jack_connection_manager.connect(
        &audio_interface.output_channels()[2 * 7],
        main_input_mixer_section.channel_inputs(9)[0]
    );
    jack_connection_manager.connect(
        &audio_interface.output_channels()[2 * 7 + 1],
        main_input_mixer_section.channel_inputs(9)[1]
    );
    jack_connection_manager.connect(
        &audio_interface.output_channels()[2 * 7],
        main_input_mixer_section.channel_inputs(9)[2]
    );
    jack_connection_manager.connect(
        &audio_interface.output_channels()[2 * 7 + 1],
        main_input_mixer_section.channel_inputs(9)[3]
    );
}

fn connect_digital_drum_section(jack_connection_manager: &JackConnectionManager,
                                digital_drum_section: &DigitalDrumSection,
                                main_input_mixer_section: &MainInputMixerChannel) {
    jack_connection_manager.connect(
        &digital_drum_section.output_channels()[0],
        main_input_mixer_section.channel_inputs(0)[0],
    );
    jack_connection_manager.connect(
        &digital_drum_section.output_channels()[1],
        main_input_mixer_section.channel_inputs(0)[1],
    );
    jack_connection_manager.connect(
        &digital_drum_section.output_channels()[0],
        main_input_mixer_section.channel_inputs(0)[2],
    );
    jack_connection_manager.connect(
        &digital_drum_section.output_channels()[1],
        main_input_mixer_section.channel_inputs(0)[3],
    );
}
