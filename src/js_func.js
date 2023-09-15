const js_loop = (packings) => {
  let cacheControlPoints = {}
        let cacheCompanies = {}

        const data = []
        for (const [i, packing] of packings.entries()) {
            // let object_temp = {}
            // let current_control_point = null

            // object_temp.current_control_point_name = '-'
            // object_temp.current_control_point_type = '-'

            // if (packing.last_event_record) {
            //     if (packing.last_event_record.type == 'inbound') {
            //         if (
            //             Object.keys(cacheControlPoints).includes(
            //                 packing.last_event_record.control_point.toString()
            //             )
            //         ) {
            //             current_control_point =
            //                 cacheControlPoints[
            //                     packing.last_event_record.control_point.toString()
            //                 ]
            //         } else {
            //             // current_control_point = await ControlPoint.findById(
            //             //     packing.last_event_record.control_point,
            //             //     'name type'
            //             // ).populate('type')

            //             cacheControlPoints[
            //                 packing.last_event_record.control_point.toString()
            //             ] = current_control_point
            //         }

            //         if (current_control_point) {
            //             object_temp.current_control_point_name =
            //                 current_control_point.name
            //             object_temp.current_control_point_type =
            //                 current_control_point.type
            //                     ? current_control_point.type.name
            //                     : '-'
            //         }
            //     } else {
            //         object_temp.current_control_point_name =
            //             current_control_point
            //                 ? current_control_point.name
            //                 : '-'
            //         object_temp.current_control_point_type =
            //             current_control_point
            //                 ? current_control_point.type.name
            //                 : '-'
            //     }
            // }

            // if (
            //     Object.keys(cacheCompanies).includes(
            //         packing.family.company.toString()
            //     )
            // ) {
            //     company = cacheCompanies[packing.family.company.toString()]
            // } else {
            //     // company = await Company.findById(packing.family.company, 'name')
            //     // cacheCompanies[packing.family.company.toString()] = company
            //     company = null
            // }

            // object_temp._id = packing._id
            // object_temp.tag = packing.tag.code
            // object_temp.family_code = packing.family ? packing.family.code : '-'
            // object_temp.serial = packing.serial
            // object_temp.company = company ? company.name : '-'
            // object_temp.current_state = packing.current_state

            // //dados do último inbound/outbound
            // object_temp.in_out_accuracy = packing.last_event_record
            //     ? packing.last_event_record.accuracy
            //     : '-'
            // object_temp.in_out_date = packing.last_event_record
            //     ? packing.last_event_record.created_at
            //     : '-'

            // //dados atuais
            // object_temp.accuracy = packing.last_position
            //     ? packing.last_position.accuracy
            //     : '-'
            // object_temp.date = packing.last_message_signal
            //     ? packing.last_message_signal
            //     : '-'

            // object_temp.battery_percentage = packing.last_battery
            //     ? packing.last_battery.battery
            //     : '-'
            // object_temp.battery_date = packing.last_battery
            //     ? packing.last_battery.date
            //     : '-'

            data.push(packing)
        }

        return data

}

module.exports = js_loop